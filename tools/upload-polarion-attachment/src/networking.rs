use crate::api_types::{get_attachments, patch_attachment, post_attachments};
use crate::documents::Document;
use anyhow::Result;
use colored::Colorize;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::to_string;
use std::path::PathBuf;

/// Control characters for path components that require escaping (https://url.spec.whatwg.org/#path-percent-encode-set)
const PATH_COMPONENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/');

/// Creates a new reqwest client with appropriate headers and configuration
pub(crate) fn create_client(token: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();
    let mut authorization = HeaderValue::from_str(&format!("Bearer {}", token))?;
    authorization.set_sensitive(true);
    headers.insert(AUTHORIZATION, authorization);

    Ok(reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()?)
}

/// Checks if an attachment already exists in Polarion
pub(crate) fn is_attachment_existing(
    document: &Document,
    name: &str,
    client: &Client,
) -> Result<bool> {
    let id = document.get_attachment_id(name);
    let attachments = get_attachments(document, client)?;
    Ok(attachments.iter().any(|a| a.id == id))
}

/// Fetches all existing attachments for a given document
pub(crate) fn get_attachments(
    document: &Document,
    client: &Client,
) -> Result<Vec<get_attachments::Attachment>> {
    let mut url = Some(format!(
        "https://pet.gematik.de/polarion/rest/v1/projects/{}/spaces/{}/documents/{}/attachments",
        document.project(),
        document.space(),
        document.document()
    ));

    let mut attachments: Vec<get_attachments::Attachment> = vec![];

    while url.is_some() {
        let unwrapped = url.unwrap();

        log_request("GET", &unwrapped);
        let response = client.get(unwrapped).send()?;
        log_response(&response);

        let mut json = response.json::<get_attachments::Response>()?;
        attachments.append(&mut json.data);

        url = json.links.and_then(|l| l.next);
    }

    Ok(attachments)
}

/// Creates a new attachments by uploading a file
pub(crate) fn post_attachment(
    path: &PathBuf,
    document: &Document,
    name: &str,
    client: &Client,
) -> Result<()> {
    let url = format!(
        "https://pet.gematik.de/polarion/rest/v1/projects/{}/spaces/{}/documents/{}/attachments",
        document.project(),
        document.space(),
        document.document()
    );

    let mut form = reqwest::blocking::multipart::Form::new();

    let resource = to_string(&post_attachments::Resource::create(name))?;
    form = form.text("resource", resource);

    let file = reqwest::blocking::multipart::Part::file(path)?.file_name(name.to_string());
    form = form.part("files", file);

    log_request("POST", &url);
    let response = client.post(url).multipart(form).send()?;
    log_response(&response);

    response.error_for_status()?;

    Ok(())
}

/// Replaces an existing attachment by uploading a file
pub(crate) fn patch_attachment(
    path: &PathBuf,
    document: &Document,
    name: &str,
    client: &Client,
) -> Result<()> {
    let url = format!(
        "https://pet.gematik.de/polarion/rest/v1/projects/{}/spaces/{}/documents/{}/attachments/{}",
        document.project(),
        document.space(),
        document.document(),
        utf8_percent_encode(name, PATH_COMPONENT).collect::<String>()
    );

    let mut form = reqwest::blocking::multipart::Form::new();

    let resource = to_string(&patch_attachment::Resource::create(
        &document.get_attachment_id(name),
        name,
    ))?;
    form = form.text("resource", resource);

    let file = reqwest::blocking::multipart::Part::file(path)?.file_name(name.to_string());
    form = form.part("files", file);

    log_request("PATCH", &url);
    let response = client.patch(url).multipart(form).send()?;
    log_response(&response);

    response.error_for_status()?;

    Ok(())
}

fn log_request(operation: &str, url: &str) {
    println!(
        "{} {}",
        operation.truecolor(128, 128, 128),
        url.truecolor(128, 128, 128)
    );
}

fn log_response(response: &Response) {
    if response.status().is_success() {
        println!("{}", response.status().to_string().green());
    } else {
        eprintln!("{}", response.status().to_string().red());
    }
}
