use crate::api::{post, Completion, Config, Message, Response};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub messages: Vec<Message>,
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            config: Config::new().unwrap(),
        }
    }
    pub fn execute(&mut self, value: &str) -> String {
        let message = Message::send(value.to_string());
        self.messages.push(message.clone());
        let mut messages = self.messages.clone();
        let config = self.config.clone();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let res = rt.block_on(async move {
            let res = post(
                "chat/completions",
                config,
                Completion {
                    model: "gpt-4".to_string(),
                    messages,
                    temperature: 0.7,
                },
            )
            .await
            .unwrap();
            let res: Response = serde_json::from_value(res).unwrap();
            res
        });

        let message = res.choices[0].message.clone();
        self.messages.push(message.clone());
        message.content
    }
}
