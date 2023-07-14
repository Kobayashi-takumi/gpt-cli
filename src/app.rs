use crate::api::{post, Completion, Config, Message, Response};
use anyhow::Result;
use log::info;

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub messages: Vec<Message>,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            messages: vec![],
            config: Config::new()?,
        })
    }
    pub fn execute(&mut self, value: &str) -> Result<String> {
        let message = Message::send(value.to_string());
        self.messages.push(message.clone());
        info!("{}", serde_json::to_string(&message)?.as_str());
        let mut messages = self.messages.clone();
        let config = self.config.clone();
        let rt = tokio::runtime::Runtime::new()?;
        let res = rt.block_on(async move { Self::api(config, messages).await })?;
        let message = res.choices[0].message.clone();
        self.messages.push(message.clone());
        info!("{}", serde_json::to_string(&message)?.as_str());
        Ok(message.content)
    }
    pub fn get_model(&self) -> String {
        self.config.model.clone()
    }
    async fn api(config: Config, messages: Vec<Message>) -> Result<Response> {
        let model = config.model.clone();
        let res = post(
            "chat/completions",
            config,
            Completion {
                model,
                messages,
                temperature: 0.7,
            },
        )
        .await?;
        let res: Response = serde_json::from_value(res).unwrap();
        Ok(res)
    }
}
