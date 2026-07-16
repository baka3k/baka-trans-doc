use crate::error::{AppError, AppResult};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

const MAX_PARTS: usize = 4_096;
const MAX_PART_BYTES: u64 = 128 * 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Clone)]
pub struct PackageEntry {
    pub name: String,
    pub data: Vec<u8>,
    pub compression: CompressionMethod,
    pub unix_mode: Option<u32>,
}

#[derive(Clone)]
pub struct OoxmlPackage {
    pub entries: Vec<PackageEntry>,
}

impl OoxmlPackage {
    pub fn open(path: &Path) -> AppResult<Self> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        if archive.len() > MAX_PARTS {
            return Err(AppError::ResourceLimit(format!(
                "{} package parts exceeds the {MAX_PARTS} part limit",
                archive.len()
            )));
        }

        let mut entries = Vec::with_capacity(archive.len());
        let mut total_bytes = 0_u64;
        for index in 0..archive.len() {
            let mut item = archive.by_index(index)?;
            let enclosed = item.enclosed_name().ok_or_else(|| {
                AppError::InvalidDocument(format!("unsafe ZIP path: {}", item.name()))
            })?;
            if enclosed.is_absolute()
                || enclosed
                    .components()
                    .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                return Err(AppError::InvalidDocument(format!(
                    "unsafe ZIP path: {}",
                    item.name()
                )));
            }
            if item.size() > MAX_PART_BYTES {
                return Err(AppError::ResourceLimit(format!(
                    "part {} exceeds {} MiB",
                    item.name(),
                    MAX_PART_BYTES / 1024 / 1024
                )));
            }
            total_bytes = total_bytes.saturating_add(item.size());
            if total_bytes > MAX_TOTAL_BYTES {
                return Err(AppError::ResourceLimit(format!(
                    "decompressed package exceeds {} MiB",
                    MAX_TOTAL_BYTES / 1024 / 1024
                )));
            }
            let mut data = Vec::with_capacity(item.size() as usize);
            item.read_to_end(&mut data)?;
            entries.push(PackageEntry {
                name: item.name().replace('\\', "/"),
                data,
                compression: item.compression(),
                unix_mode: item.unix_mode(),
            });
        }

        let package = Self { entries };
        if package.entry("[Content_Types].xml").is_none() {
            return Err(AppError::InvalidDocument(
                "missing [Content_Types].xml".into(),
            ));
        }
        Ok(package)
    }

    pub fn entry(&self, name: &str) -> Option<&PackageEntry> {
        self.entries.iter().find(|entry| entry.name == name)
    }

    pub fn entry_mut(&mut self, name: &str) -> Option<&mut PackageEntry> {
        self.entries.iter_mut().find(|entry| entry.name == name)
    }

    pub fn write(&self, path: &Path) -> AppResult<()> {
        let file = File::create(path)?;
        let mut archive = ZipWriter::new(file);
        for entry in &self.entries {
            let mut options = SimpleFileOptions::default().compression_method(entry.compression);
            if let Some(mode) = entry.unix_mode {
                options = options.unix_permissions(mode);
            }
            if entry.name.ends_with('/') {
                archive.add_directory(&entry.name, options)?;
            } else {
                archive.start_file(&entry.name, options)?;
                archive.write_all(&entry.data)?;
            }
        }
        archive.finish()?.sync_all()?;
        Ok(())
    }
}

pub fn file_sha256(path: &Path) -> AppResult<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn sibling_temp_path(output: &Path) -> AppResult<PathBuf> {
    let parent = output.parent().ok_or_else(|| {
        AppError::UnsafeOutput("output path does not have a parent directory".into())
    })?;
    let file_name = output
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| AppError::UnsafeOutput("output file name is invalid".into()))?;
    Ok(parent.join(format!(".{file_name}.{}.tmp", uuid::Uuid::new_v4())))
}
