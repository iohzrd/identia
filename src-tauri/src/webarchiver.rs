#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use core::str;
use encoding_rs::Encoding;
use markup5ever_rcdom::RcDom;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Error, Write};
use std::path::Path;
use std::process;
use std::time::Duration;
use tauri;
use tauri_plugin_http::reqwest;
use url::Url;

use monolith::html::{
  add_favicon, create_metadata_tag, get_base_url, get_charset, has_favicon, html_to_dom,
  serialize_document, set_base_url, set_charset, walk_and_embed_assets,
};
use monolith::opts::Options;
use monolith::url::{create_data_url, resolve_url};
use monolith::utils::retrieve_asset;

enum Output {
  Stdout(io::Stdout),
  File(fs::File),
}

impl Output {
  fn new(file_path: &str) -> Result<Output, Error> {
    if file_path.is_empty() || file_path.eq("-") {
      Ok(Output::Stdout(io::stdout()))
    } else {
      Ok(Output::File(fs::File::create(file_path)?))
    }
  }

  fn write(&mut self, bytes: &Vec<u8>) -> Result<(), Error> {
    match self {
      Output::Stdout(stdout) => {
        stdout.write_all(bytes)?;
        // Ensure newline at end of output
        if bytes.last() != Some(&b"\n"[0]) {
          stdout.write(b"\n")?;
        }
        stdout.flush()
      }
      Output::File(file) => {
        file.write_all(bytes)?;
        // Ensure newline at end of output
        if bytes.last() != Some(&b"\n"[0]) {
          file.write(b"\n")?;
        }
        file.flush()
      }
    }
  }
}

#[tauri::command]
pub fn archive_webpage(url: String) -> String {
  println!("archive_webpage: {:#?}", url);
  let mut options = Options::default();
  options.isolate = true;
  options.no_audio = true;
  options.no_fonts = true;
  options.no_frames = true;
  options.no_js = true;
  options.no_video = true;
  options.unwrap_noscript = true;

  let target_url = match url.as_str() {
    target => match Url::parse(&target) {
      Ok(url) => match url.scheme() {
        "data" | "file" | "http" | "https" => url,
        unsupported_scheme => {
          eprintln!("Unsupported target URL type: {}", unsupported_scheme);
          Url::parse("").unwrap()
        }
      },
      Err(_) => {
        // Failed to parse given base URL (perhaps it's a filesystem path?)
        let path: &Path = Path::new(&target);
        match path.exists() {
          true => match path.is_file() {
            true => {
              let canonical_path = fs::canonicalize(&path).unwrap();
              match Url::from_file_path(canonical_path) {
                Ok(url) => url,
                Err(_) => {
                  eprintln!("Could not generate file URL out of given path: {}", &target);
                  Url::parse("").unwrap()
                }
              }
            }
            false => {
              eprintln!("Local target is not a file: {}", &target);
              Url::parse("").unwrap()
            }
          },
          false => {
            // It is not a FS path, now we do what browsers do:
            // prepend "http://" and hope it points to a website
            Url::parse(&format!("http://{hopefully_url}", hopefully_url = &target)).unwrap()
          }
        }
      }
    },
  };

  // Initialize client
  let mut cache = HashMap::new();
  let mut header_map = HeaderMap::new();
  if let Some(user_agent) = &options.user_agent {
    header_map.insert(
      USER_AGENT,
      HeaderValue::from_str(&user_agent).expect("Invalid User-Agent header specified"),
    );
  }

  let client = Client::builder()
    .timeout(Duration::from_secs(30))
    .danger_accept_invalid_certs(options.insecure)
    .default_headers(header_map)
    .build()
    .expect("Failed to initialize HTTP client");

  // At first we assume that base URL is the same as target URL
  let mut base_url: Url = target_url.clone();

  let mut data: Vec<u8> = Vec::new();
  let mut document_encoding: String = "".to_string();
  let mut dom: RcDom;

  // Retrieve target document
  if target_url.scheme() == "file"
    || (target_url.scheme() == "http" || target_url.scheme() == "https")
    || target_url.scheme() == "data"
  {
    match retrieve_asset(&mut cache, &client, &target_url, &target_url, &options) {
      Ok((retrieved_data, final_url, media_type, charset)) => {
        // Provide output as text without processing it, the way browsers do
        if !media_type.eq_ignore_ascii_case("text/html")
          && !media_type.eq_ignore_ascii_case("application/xhtml+xml")
        {
          // Define output
          let mut output = Output::new(&options.output).expect("Could not prepare output");

          // Write retrieved data into STDOUT or file
          output
            .write(&retrieved_data)
            .expect("Could not write output");
        }

        if options
          .base_url
          .clone()
          .unwrap_or("".to_string())
          .is_empty()
        {
          base_url = final_url;
        }

        data = retrieved_data;
        document_encoding = charset;
      }
      Err(_) => {
        eprintln!("Could not retrieve target document");
      }
    }
  }

  // Initial parse
  dom = html_to_dom(&data, document_encoding.clone());

  // TODO: investigate if charset from filesystem/data URL/HTTP headers
  //       has say over what's specified in HTML

  // Attempt to determine document's charset
  if let Some(html_charset) = get_charset(&dom.document) {
    if !html_charset.is_empty() {
      // Check if the charset specified inside HTML is valid
      if let Some(encoding) = Encoding::for_label_no_replacement(html_charset.as_bytes()) {
        document_encoding = html_charset;
        dom = html_to_dom(&data, encoding.name().to_string());
      }
    }
  }

  // Use custom base URL if specified, read and use what's in the DOM otherwise
  let custom_base_url: String = options.base_url.clone().unwrap_or("".to_string());
  if custom_base_url.is_empty() {
    // No custom base URL is specified
    // Try to see if document has BASE element
    if let Some(existing_base_url) = get_base_url(&dom.document) {
      base_url = resolve_url(&target_url, &existing_base_url);
    }
  } else {
    // Custom base URL provided
    match Url::parse(&custom_base_url) {
      Ok(parsed_url) => {
        if parsed_url.scheme() == "file" {
          // File base URLs can only work with
          // documents saved from filesystem
          if target_url.scheme() == "file" {
            base_url = parsed_url;
          }
        } else {
          base_url = parsed_url;
        }
      }
      Err(_) => {
        // Failed to parse given base URL, perhaps it's a filesystem path?
        if target_url.scheme() == "file" {
          // Relative paths could work for documents saved from filesystem
          let path: &Path = Path::new(&custom_base_url);
          if path.exists() {
            match Url::from_file_path(fs::canonicalize(&path).unwrap()) {
              Ok(file_url) => {
                base_url = file_url;
              }
              Err(_) => {
                eprintln!("Could not map given path to base URL: {}", custom_base_url);
              }
            }
          }
        }
      }
    }
  }

  // Traverse through the document and embed remote assets
  walk_and_embed_assets(&mut cache, &client, &base_url, &dom.document, &options);

  // Update or add new BASE element to reroute network requests and hash-links
  if let Some(new_base_url) = options.base_url.clone() {
    dom = set_base_url(&dom.document, new_base_url);
  }

  // Request and embed /favicon.ico (unless it's already linked in the document)
  if !options.no_images
    && (target_url.scheme() == "http" || target_url.scheme() == "https")
    && !has_favicon(&dom.document)
  {
    let favicon_ico_url: Url = resolve_url(&base_url, "/favicon.ico");

    match retrieve_asset(&mut cache, &client, &target_url, &favicon_ico_url, &options) {
      Ok((data, final_url, media_type, charset)) => {
        let favicon_data_url: Url = create_data_url(&media_type, &charset, &data, &final_url);
        dom = add_favicon(&dom.document, favicon_data_url.to_string());
      }
      Err(_) => {
        // Failed to retrieve /favicon.ico
      }
    }
  }

  // Save using specified charset, if given
  if let Some(custom_encoding) = options.encoding.clone() {
    document_encoding = custom_encoding;
    dom = set_charset(dom, document_encoding.clone());
  }

  // Serialize DOM tree
  let mut result: Vec<u8> = serialize_document(dom, document_encoding, &options);

  // Prepend metadata comment tag
  if !options.no_metadata {
    let mut metadata_comment: String = create_metadata_tag(&target_url);
    metadata_comment += "\n";
    result.splice(0..0, metadata_comment.as_bytes().to_vec());
  }

  String::from_utf8(result).unwrap_or(String::new())
}
