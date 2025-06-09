pub mod error;

use super::BackendError;
use crate::error::CoreError;
use crate::item::ItemEntry;
use crate::parser;
use error::DocsrsBackendError;
use reqwest::blocking::Client;
use std::io::Read;
use zstd::Decoder as ZstdDecoder;

fn fetch_crate_all_items_html(
    crate_name: &str,
    crate_version: &str,
    app_user_agent: &str,
) -> Result<String, DocsrsBackendError> {
    let html_url = format!(
        "https://docs.rs/{}/{}/{}/all.html",
        crate_name, crate_version, crate_name,
    );

    let client = Client::builder()
        .user_agent(app_user_agent)
        .build()
        .map_err(DocsrsBackendError::ClientBuild)?;

    let response = client
        .get(&html_url)
        .send()
        .map_err(|e| DocsrsBackendError::HttpGet(html_url.clone(), e))?
        .error_for_status()
        .map_err(|e| DocsrsBackendError::HttpStatus(html_url.clone(), e))?;

    eprintln!("final URL: {}", response.url());
    if let Some(ct) = response.headers().get(reqwest::header::CONTENT_TYPE) {
        eprintln!("Content-Type: {:?}", ct);
    }
    if let Some(enc) = response.headers().get(reqwest::header::CONTENT_ENCODING) {
        eprintln!("Content-Encoding: {:?}", enc);
    }

    let content_encoding = response
        .headers()
        .get(reqwest::header::CONTENT_ENCODING)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_ascii_lowercase());

    let raw_bytes = response
        .bytes()
        .map_err(|e| DocsrsBackendError::HttpGet(html_url.clone(), e))?;

    let html_bytes: Vec<u8> = match content_encoding.as_deref() {
        Some("zstd") => {
            let mut decoder =
                ZstdDecoder::new(raw_bytes.as_ref()).map_err(DocsrsBackendError::Decompress)?;
            let mut buf = Vec::new();
            decoder
                .read_to_end(&mut buf)
                .map_err(DocsrsBackendError::Decompress)?;
            buf
        },
        _ => raw_bytes.to_vec(),
    };

    std::str::from_utf8(&html_bytes)
        .map(|s| s.to_string())
        .map_err(DocsrsBackendError::Utf8)
}

pub fn process(
    crate_name: &str,
    crate_version: &str,
    app_user_agent: &str,
) -> Result<Vec<ItemEntry>, CoreError> {
    let html_content = fetch_crate_all_items_html(crate_name, crate_version, app_user_agent)
        .map_err(BackendError::from)
        .map_err(CoreError::from)?;
    parser::parse_html_to_items(crate_name, &html_content).map_err(CoreError::from)
}
