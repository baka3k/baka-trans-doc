use super::*;
use std::{fs::File, io::Write};
use tempfile::tempdir;
use zip::{ZipWriter, write::SimpleFileOptions};

const CONTENT_TYPES_DOCX: &str = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
const CONTENT_TYPES_PPTX: &str = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/><Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/></Types>"#;
const CONTENT_TYPES_XLSX: &str = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/><Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/></Types>"#;

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

    let mut document = DocumentSession::open(&path).unwrap();
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
    let document = DocumentSession::open(&path).unwrap();
    assert_eq!(document.units().len(), 1);
    assert_eq!(document.units()[0].location.slide, Some(1));
    assert_eq!(document.units()[0].text, "発表資料");
}

#[test]
fn xlsx_translates_shared_and_inline_strings_without_touching_formulas_or_media() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("minimal.xlsx");
    let workbook = br#"<workbook xmlns:r="r"><sheets><sheet name="Data" sheetId="1" r:id="rId1"/></sheets></workbook>"#;
    let relationships = br#"<Relationships><Relationship Id="rId1" Type="worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#;
    let shared = br#"<sst><si><r><rPr><b/></rPr><t>Red</t></r><r><t>Blue</t></r></si></sst>"#;
    let sheet = br#"<worksheet><sheetData><row r="1"><c r="A1" t="s" s="2"><v>0</v></c><c r="A2" t="s"><v>0</v></c><c r="B1" t="inlineStr"><is><t>Hello</t></is></c><c r="C1"><f>SUM(1,2)</f><v>3</v></c></row></sheetData></worksheet>"#;
    let media = b"unchanged-image-sentinel";
    write_package(
        &path,
        &[
            ("[Content_Types].xml", CONTENT_TYPES_XLSX.as_bytes()),
            ("xl/workbook.xml", workbook),
            ("xl/_rels/workbook.xml.rels", relationships),
            ("xl/sharedStrings.xml", shared),
            ("xl/worksheets/sheet1.xml", sheet),
            ("xl/media/image1.png", media),
        ],
    );
    let mut document = DocumentSession::open(&path).unwrap();
    assert_eq!(document.units().len(), 2);
    assert_eq!(document.units()[0].location.label, "Data!A1");
    assert_eq!(document.units()[0].text, "⟦S0⟧Red⟦S1⟧Blue");
    let shared_id = document.units()[0].id.clone();
    let inline_id = document.units()[1].id.clone();
    document.set_translation(&shared_id, "⟦S0⟧Đỏ⟦S1⟧Xanh".into());
    document.set_translation(&inline_id, "Xin chào".into());
    let output = document.translated_package().unwrap();
    let shared_out =
        String::from_utf8(output.entry("xl/sharedStrings.xml").unwrap().data.clone()).unwrap();
    let sheet_out = String::from_utf8(
        output
            .entry("xl/worksheets/sheet1.xml")
            .unwrap()
            .data
            .clone(),
    )
    .unwrap();
    assert!(shared_out.contains("<b/><"));
    assert!(shared_out.contains("<t>Đỏ</t>"));
    assert!(shared_out.contains("<t>Xanh</t>"));
    assert!(sheet_out.contains("<t>Xin chào</t>"));
    assert!(sheet_out.contains("<f>SUM(1,2)</f><v>3</v>"));
    assert_eq!(output.entry("xl/media/image1.png").unwrap().data, media);
}

#[test]
fn output_name_uses_the_canonical_target_and_preserves_markdown_extension() {
    assert_eq!(
        output_file_name(Path::new("report.markdown"), "ZH-hant").unwrap(),
        "report_zh-Hant.markdown"
    );
}

#[test]
fn text_export_is_atomic_keeps_input_immutable_and_refuses_collision() {
    let directory = tempdir().unwrap();
    let input = directory.path().join("note.txt");
    let output = directory.path().join("note_en.txt");
    std::fs::write(&input, "  hello\r\n").unwrap();
    let mut document = DocumentSession::open(&input).unwrap();
    let id = document.units()[0].id.clone();
    document.set_translation(&id, "welcome".into());
    document.export_atomic(&input, &output).unwrap();
    assert_eq!(std::fs::read_to_string(&input).unwrap(), "  hello\r\n");
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "  welcome\r\n");

    let document = DocumentSession::open(&input).unwrap();
    assert!(matches!(
        document.export_atomic(&input, &output),
        Err(AppError::OutputExists(_))
    ));
}
