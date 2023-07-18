use super::Command;
use crate::api::{
    model::{CompletionRequest, CompletionResponse, ImageRequest, ImageResponse, Message},
    post, Config,
};
use anyhow::Result;
use log::info;

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub messages: Vec<Message>,
    config: Config,
    command: Command,
}

impl App {
    pub fn new(command: Command) -> Result<Self> {
        Ok(Self {
            messages: vec![],
            config: Config::new()?,
            command,
        })
    }
    pub fn execute(&mut self, value: &str) -> Result<String> {
        match self.command {
            Command::Chat => self.chat(value),
            Command::Image => self.image(value),
        }
    }
    pub fn get_model(&self) -> String {
        self.config.model.clone()
    }
    ///
    /// チャット
    ///
    fn chat(&mut self, value: &str) -> Result<String> {
        let message = Message::send(value.to_string());
        self.messages.push(message.clone());
        info!("{}", serde_json::to_string(&message)?.as_str());
        let mut messages = self.messages.clone();
        let config = self.config.clone();
        let rt = tokio::runtime::Runtime::new()?;
        let res = rt.block_on(async move { Self::chat_api(config, messages).await })?;
        let message = res.choices[0].message.clone();
        self.messages.push(message.clone());
        info!("{}", serde_json::to_string(&message)?.as_str());
        Ok(message.content)
    }
    async fn chat_api(config: Config, messages: Vec<Message>) -> Result<CompletionResponse> {
        let model = config.model.clone();
        let res = post(
            "chat/completions",
            config,
            CompletionRequest {
                model,
                messages,
                temperature: 0.7,
            },
        )
        .await?;
        let res = serde_json::from_value(res)?;
        Ok(res)
    }
    ///
    /// 画像
    ///
    fn image(&mut self, value: &str) -> Result<String> {
        let image = ImageRequest::sender(value.to_string());
        info!("{}", serde_json::to_string(&image)?.as_str());
        let config = self.config.clone();

        let rt = tokio::runtime::Runtime::new()?;
        let res = rt.block_on(async move { Self::image_api(config, image).await })?;
        info!("{}", serde_json::to_string(&res)?);
        Ok(res
            .data
            .into_iter()
            .map(|i| i.url)
            .collect::<Vec<String>>()
            .join(","))
    }
    async fn image_api(config: Config, image: ImageRequest) -> Result<ImageResponse> {
        let res = post("images/generations", config, image).await?;
        let res = serde_json::from_value(res)?;
        Ok(res)
    }
}
