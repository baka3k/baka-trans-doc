use super::*;
use std::{fs::File, io::Write};
use tempfile::tempdir;
use zip::{ZipWriter, write::SimpleFileOptions};

const CONTENT_TYPES_DOCX: &str = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
const CONTENT_TYPES_PPTX: &str = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/><Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/></Types>"#;

fn write_package(path: &Path, parts: &[(&str, &[u8])]) {
    let file = File::create(path).unwrap();
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default();
    for (name, data) in parts {
        zip.start_file(name, options).unwrap();
        zip.write_all(data).unwrap();
    }
    zip.finish().unwrap();
}

#[test]
fn docx_translation_only_mutates_mapped_text_part() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("minimal.docx");
    let document_xml = include_bytes!("../../../tests/fixtures/docx/minimal-document.xml");
    let media = b"unchanged-image-sentinel";
    write_package(
        &path,
        &[
            ("[Content_Types].xml", CONTENT_TYPES_DOCX.as_bytes()),
            ("word/document.xml", document_xml),
            ("word/media/image1.png", media),
        ],
    );

    let mut document = DocumentPackage::open(&path).unwrap();
    assert_eq!(document.units().len(), 1);
    let unit_id = document.units()[0].id.clone();
    document.set_translation(&unit_id, "⟦S0⟧đỏ⟦S1⟧xanh".into());
    let output = document.translated_package().unwrap();
    let xml = String::from_utf8(output.entry("word/document.xml").unwrap().data.clone()).unwrap();
    assert!(xml.contains("<w:t>đỏ</w:t>"));
    assert!(xml.contains("<w:b/>"));
    assert_eq!(output.entry("word/media/image1.png").unwrap().data, media);
    assert_eq!(
        output.entry("[Content_Types].xml").unwrap().data,
        CONTENT_TYPES_DOCX.as_bytes()
    );
}

#[test]
fn extracts_pptx_slide_units() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("minimal.pptx");
    let slide_xml = include_bytes!("../../../tests/fixtures/pptx/minimal-slide.xml");
    write_package(
        &path,
        &[
            ("[Content_Types].xml", CONTENT_TYPES_PPTX.as_bytes()),
            ("ppt/presentation.xml", b"<p:presentation xmlns:p=\"p\"/>"),
            ("ppt/slides/slide1.xml", slide_xml),
        ],
    );
    let document = DocumentPackage::open(&path).unwrap();
    assert_eq!(document.units().len(), 1);
    assert_eq!(document.units()[0].location.slide, Some(1));
    assert_eq!(document.units()[0].text, "発表資料");
}
