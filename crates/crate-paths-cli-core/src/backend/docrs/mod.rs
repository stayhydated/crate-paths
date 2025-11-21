pub mod error;

use super::BackendError;
use crate::error::CoreError;
use crate::item::ItemEntry;
use crate::parser;
use error::DocsrsBackendError;
use reqwest::blocking::Client;
use std::io::Read as _;
use zstd::Decoder as ZstdDecoder;

fn fetch_crate_all_items_html(
    crate_name: &str,
    crate_version: &str,
    app_user_agent: &str,
) -> Result<String, DocsrsBackendError> {
    let base_url = format!("https://docs.rs/{}/{}/", crate_name, crate_version,);

    let client = Client::builder()
        .user_agent(app_user_agent)
        .build()
        .map_err(DocsrsBackendError::ClientBuild)?;

    // Perform a HEAD request to follow redirects and get the final URL
    let response = client
        .head(&base_url)
        .send()
        .map_err(|e| DocsrsBackendError::HttpGet(base_url.clone(), e))?
        .error_for_status()
        .map_err(|e| DocsrsBackendError::HttpStatus(base_url.clone(), e))?;

    let final_url = response.url();
    let html_url = final_url
        .join("all.html")
        .expect("failed to construct all.html URL");
    let html_url_str = html_url.to_string();

    let response = client
        .get(html_url)
        .send()
        .map_err(|e| DocsrsBackendError::HttpGet(html_url_str.clone(), e))?
        .error_for_status()
        .map_err(|e| DocsrsBackendError::HttpStatus(html_url_str.clone(), e))?;

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
        .map_err(|e| DocsrsBackendError::HttpGet(html_url_str.clone(), e))?;

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
