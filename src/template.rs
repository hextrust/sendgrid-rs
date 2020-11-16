//! Sendgrid APIs related to templates
//!
use std::collections::HashMap;

use crate::error::{RequestNotSuccessful, SendgridResult};
use data_encoding::BASE64;
use reqwest::header::{self, HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use reqwest::{Client, Response};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Templates {
    templates: Vec<Template>,
}

/// Template
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Template {
    /// ID
    pub id: String,
    /// Name
    pub name: String,
    /// Generation
    pub generation: Option<String>,
    /// Last updated
    pub updated_at: Option<String>,
    /// versions
    pub versions: Vec<TemplateShortVersion>,
}

/// specific template versions
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateShortVersion {
    /// ID
    pub id: String,
    /// Name
    pub name: String,
    /// Template ID
    pub template_id: String,
    /// Active
    pub active: u32,
    /// Editor
    pub editor: String,
    /// Subject
    pub subject: String,
    /// Last updated
    pub updated_at: String,
}

/// specific template versions
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateVersion {
    /// ID
    pub id: String,
    /// Template ID
    pub template_id: String,
    /// Last updated
    pub updated_at: Option<String>,
    /// Last updated
    pub thumbnail_url: Option<String>,
    /// User ID
    pub user_id: Option<u32>,
    /// Active
    pub active: u32,
    /// Name
    pub name: String,
    /// Html Content
    pub html_content: Option<String>,
    /// Plain Content
    pub plain_content: Option<String>,
    /// Generate plain content
    pub generate_plain_content: Option<bool>,
    /// Subject
    pub subject: String,
    /// Editor
    pub editor: String,
    /// Test data
    pub test_data: Option<String>,
    /// Warnings
    pub warnings: Option<Vec<WarningMessage>>,
}

/// Warning Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WarningMessage {
    message: String,
}

const BASE_URL: &str = "https://api.sendgrid.com/";

fn get_headers(api_key: &str) -> Result<HeaderMap, InvalidHeaderValue> {
    let mut headers = HeaderMap::with_capacity(3);
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(header::USER_AGENT, HeaderValue::from_static("sendgrid-rs"));
    Ok(headers)
}

fn base_url() -> Url {
    Url::parse(BASE_URL).unwrap()
}

/// Create a new template
pub async fn create(api_key: &str, template_name: String) -> SendgridResult<String> {
    let mut url = base_url();
    url.set_path("/v3/templates");

    let headers = get_headers(api_key)?;

    #[derive(Debug, Serialize)]
    struct Query<'a> {
        name: String,
        generation: &'a str,
    };

    let q = Query {
        name: template_name,
        generation: "dynamic",
    };

    let resp = Client::new()
        .post(url)
        .headers(headers)
        .body(serde_json::to_string(&q).unwrap())
        .send()
        .await?;

    if let Err(_) = resp.error_for_status_ref() {
        return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
    }
    let r: Template = resp.json().await?;
    //println!("{:?}", r);
    Ok(r.id)
}

/// List all templates
pub async fn list(api_key: &str) -> SendgridResult<Vec<Template>> {
    let mut url = base_url();
    url.set_path("/v3/templates");
    url.query_pairs_mut().append_pair("generations", "dynamic");

    let headers = get_headers(api_key)?;

    let resp = Client::new().get(url).headers(headers).send().await?;

    if let Err(_) = resp.error_for_status_ref() {
        return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
    }
    let r: Templates = resp.json().await?;
    Ok(r.templates)
}

/// Get a template specific version
pub async fn get_version(
    api_key: &str,
    template_name: &str,
    version: &str,
) -> SendgridResult<TemplateVersion> {
    let mut url = base_url();
    url.set_path(&format!(
        "/v3/templates/{}/versions/{}",
        template_name, version
    ));

    let headers = get_headers(api_key)?;

    let resp = Client::new().get(url).headers(headers).send().await?;

    if let Err(_) = resp.error_for_status_ref() {
        return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
    }
    let response_body = resp.bytes().await?;
    let r: TemplateVersion = serde_json::from_slice(&response_body)?;
    Ok(r)
}

/// Get a template specific version
pub async fn add_version(api_key: &str, template_id: &str) -> SendgridResult<TemplateVersion> {
    let mut url = base_url();
    url.set_path(&format!("/v3/templates/{}/versions", template_id));

    let headers = get_headers(api_key)?;

    #[derive(Debug, Serialize)]
    struct Query<'a> {
        template_id: &'a str,
        active: u32,
        name: &'a str,
        html_content: &'a str,
        plain_content: &'a str,
        subject: &'a str,
    };

    let q = Query {
        template_id,
        active: 1,
        name: "name",
        html_content: "html_content",
        plain_content: "plain_content",
        subject: "my subject",
    };

    let resp = Client::new()
        .post(url)
        .headers(headers)
        .body(serde_json::to_string(&q).unwrap())
        .send()
        .await?;

    if let Err(_) = resp.error_for_status_ref() {
        return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
    }
    let response_body = resp.bytes().await?;
    //println!(
    //    "{}",
    //    String::from_utf8(response_body.as_ref().to_vec()).unwrap()
    //);
    let r: TemplateVersion = serde_json::from_slice(&response_body)?;
    Ok(r)
}
