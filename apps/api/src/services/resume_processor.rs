use std::io::{Cursor, Read};

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

const CHUNK_SIZE: usize = 1500;
const CHUNK_OVERLAP: usize = 200;

pub fn chunk_text(text: &str) -> Vec<String> {
    let cleaned: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned.is_empty() {
        return vec![];
    }
    if cleaned.len() <= CHUNK_SIZE {
        return vec![cleaned];
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    while start < cleaned.len() {
        let end = (start + CHUNK_SIZE).min(cleaned.len());
        let mut chunk_end = end;
        if end < cleaned.len() {
            if let Some(pos) = cleaned[start..end].rfind(' ') {
                chunk_end = start + pos;
            }
        }
        chunks.push(cleaned[start..chunk_end].trim().to_string());
        if chunk_end >= cleaned.len() {
            break;
        }
        start = chunk_end.saturating_sub(CHUNK_OVERLAP);
    }
    chunks.retain(|c| !c.is_empty());
    chunks
}

pub fn extract_text_from_bytes(filename: &str, bytes: &[u8]) -> anyhow::Result<String> {
    let lower = filename.to_lowercase();
    if lower.ends_with(".txt") || lower.ends_with(".md") {
        return Ok(String::from_utf8_lossy(bytes).to_string());
    }
    if lower.ends_with(".pdf") {
        return pdf_extract::extract_text_from_mem(bytes)
            .map_err(|e| anyhow::anyhow!("could not extract PDF text: {e}"));
    }
    if lower.ends_with(".docx") {
        return extract_docx_text(bytes);
    }
    if lower.ends_with(".doc") {
        anyhow::bail!(
            "legacy .doc files are not supported; save the resume as .docx, .pdf, or .txt"
        );
    }
    Ok(String::from_utf8_lossy(bytes).to_string())
}

fn extract_docx_text(bytes: &[u8]) -> anyhow::Result<String> {
    let mut archive = ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| anyhow::anyhow!("could not open DOCX archive: {e}"))?;
    let mut document = archive
        .by_name("word/document.xml")
        .map_err(|e| anyhow::anyhow!("DOCX has no word/document.xml: {e}"))?;
    let mut xml = String::new();
    document.read_to_string(&mut xml)?;

    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut output = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Text(text)) => {
                if !output.is_empty() {
                    output.push(' ');
                }
                output.push_str(&text.decode()?);
            }
            Ok(Event::End(tag)) if tag.name().as_ref() == b"w:p" => output.push('\n'),
            Ok(Event::Eof) => break,
            Err(e) => anyhow::bail!("could not parse DOCX XML: {e}"),
            _ => {}
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write};

    use zip::write::SimpleFileOptions;

    use super::{chunk_text, extract_text_from_bytes};

    #[test]
    fn extracts_text_from_docx_document_xml() {
        let mut output = Cursor::new(Vec::new());
        {
            let mut archive = zip::ZipWriter::new(&mut output);
            archive
                .start_file("word/document.xml", SimpleFileOptions::default())
                .unwrap();
            archive
                .write_all(
                    br#"<?xml version="1.0"?><w:document xmlns:w="w"><w:body><w:p><w:r><w:t>Rust engineer</w:t></w:r></w:p><w:p><w:r><w:t>Postgres</w:t></w:r></w:p></w:body></w:document>"#,
                )
                .unwrap();
            archive.finish().unwrap();
        }

        let text = extract_text_from_bytes("resume.docx", output.get_ref()).unwrap();
        assert!(text.contains("Rust engineer"));
        assert!(text.contains("Postgres"));
    }

    #[test]
    fn rejects_legacy_doc_files() {
        let error = extract_text_from_bytes("resume.doc", b"legacy").unwrap_err();
        assert!(error.to_string().contains("not supported"));
    }

    #[test]
    fn chunking_removes_empty_whitespace() {
        assert!(chunk_text(" \n\t ").is_empty());
    }
}
