use anyhow::{Ok, Result};
use etcetera::{choose_base_strategy, BaseStrategy};
use hyper::{body::to_bytes, http::request::Builder, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use toml;

const URI: &str = "https://api.openai.com/v1/";

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    key: String,
    org: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub key: String,
    pub org: String,
    pub base_uri: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let strategy = choose_base_strategy().expect("Unable to find the config directory!");
        let mut path = strategy.config_dir();
        path.push("gpt-rs");
        let path = path.join("config.toml");
        let buf: String = fs::read_to_string(path).expect("You must create config.toml in the directory ~/.config/gpt-rs. And then you must set key and org properties.");
        let setting: Setting = toml::from_str(&buf).unwrap();

        Ok(Self {
            key: setting.key,
            org: setting.org,
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
    let body_bytes = to_bytes(res.into_body()).await.unwrap();
    let res = serde_json::from_slice(&body_bytes)?;
    Ok(res)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Completion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn send(value: String) -> Self {
        Self {
            role: "user".to_string(),
            content: value,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u64,
}