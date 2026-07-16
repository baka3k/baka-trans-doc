use crate::{
    error::{AppError, AppResult},
    storage::package::{OoxmlPackage, sibling_temp_path},
};
use std::{fs, path::Path};

pub fn export_atomic(package: &OoxmlPackage, input: &Path, output: &Path) -> AppResult<()> {
    export_with(
        input,
        output,
        |temp| package.write(temp),
        |temp| {
            let validated = OoxmlPackage::open(temp)?;
            if validated.entries.len() != package.entries.len() {
                return Err(AppError::InvalidDocument(
                    "exported package lost one or more parts".into(),
                ));
            }
            Ok(())
        },
    )
}

pub fn export_bytes_atomic(bytes: &[u8], input: &Path, output: &Path) -> AppResult<()> {
    export_with(
        input,
        output,
        |temp| {
            fs::write(temp, bytes)?;
            Ok(())
        },
        |temp| {
            let written = fs::read(temp)?;
            if written != bytes {
                return Err(AppError::Io("exported file failed byte validation".into()));
            }
            let payload = written
                .strip_prefix(&[0xEF, 0xBB, 0xBF])
                .unwrap_or(&written);
            std::str::from_utf8(payload)
                .map_err(|_| AppError::InvalidDocument("exported text is not UTF-8".into()))?;
            Ok(())
        },
    )
}

fn export_with<W, V>(input: &Path, output: &Path, write: W, validate: V) -> AppResult<()>
where
    W: FnOnce(&Path) -> AppResult<()>,
    V: FnOnce(&Path) -> AppResult<()>,
{
    if output.exists() {
        return Err(AppError::OutputExists(output.display().to_string()));
    }
    let input = input.canonicalize()?;
    let parent = output.parent().ok_or_else(|| {
        AppError::UnsafeOutput("output path does not have a parent directory".into())
    })?;
    let parent = parent.canonicalize()?;
    let output_name = output
        .file_name()
        .ok_or_else(|| AppError::UnsafeOutput("output file name is invalid".into()))?;
    let resolved_output = parent.join(output_name);
    if resolved_output == input {
        return Err(AppError::UnsafeOutput(
            "the output may not overwrite the input document".into(),
        ));
    }

    let temp = sibling_temp_path(&resolved_output)?;
    let result = (|| {
        write(&temp)?;
        validate(&temp)?;
        fs::rename(&temp, &resolved_output)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}
