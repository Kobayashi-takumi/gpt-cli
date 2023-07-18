use crate::loader::config_file;
use anyhow::{anyhow, Result};
use etcetera::{choose_base_strategy, BaseStrategy};
use hyper::{body::to_bytes, http::request::Builder, Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use toml;

pub mod model;

const URI: &str = "https://api.openai.com/v1/";

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    key: String,
    org: String,
    model: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub key: String,
    pub org: String,
    pub model: String,
    pub base_uri: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = config_file()?;
        let buf: String = match fs::read_to_string(path) {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("You must create config.toml in the directory ~/.config/gpt-rs. And then you must set key and org properties."))
        };
        let setting: Setting = match toml::from_str(&buf) {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("You must create config.toml in the directory ~/.config/gpt-rs. And then you must set key and org properties.")),
        };

        Ok(Self {
            key: setting.key,
            org: setting.org,
            model: setting.model.unwrap_or("gpt-4".to_string()),
            base_uri: URI.to_string(),
        })
    }
}

fn buider(uri: &str, config: &Config) -> Builder {
    Request::builder()
        .uri(format!("{}{uri}", config.base_uri))
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {}", config.key))
        .header("OpenAI-Organization", &config.org)
}

pub async fn get(uri: &str, config: Config) -> Result<Value> {
    let request = buider(uri, &config)
        .method(Method::GET)
        .body(Body::empty())?;
    let client = Client::builder().build(HttpsConnector::new());
    let res = client.request(request).await?;
    let body_bytes = to_bytes(res.into_body()).await.unwrap();
    let res = serde_json::from_slice(&body_bytes)?;
    Ok(res)
}

pub async fn post<B: Serialize>(uri: &str, config: Config, body: B) -> Result<Value> {
    let body = Body::from(serde_json::to_string(&body)?);
    let request = buider(uri, &config).method(Method::POST).body(body)?;
    let client = Client::builder().build(HttpsConnector::new());
    let res = client.request(request).await?;
    let status = res.status();
    let body_bytes = to_bytes(res.into_body()).await?;
    let res_ = serde_json::from_slice(&body_bytes)?;
    match status.is_success() {
        true => Ok(res_),
        false => {
            let error: model::ErrorResponse = serde_json::from_value(res_)?;
            Err(anyhow!(error.error.message))
        }
    }
}
