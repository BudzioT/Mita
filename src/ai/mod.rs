use serde_json::{json, Value};
use anyhow::Result;
use reqwest;


pub struct AI {
    api_token: String,
    user_id: String,
    model: String,
    system_prompt: String,
}

impl AI {
    pub fn new(api_token: String, user_id: String, model: String, system_prompt: String) -> Self {
        Self {
            api_token,
            user_id,
            model,
            system_prompt,
        }
    }

    pub async fn generate(self: &mut Self, prompt: String) -> Result<Value> {
        let inputs = json!({
            "messages": [
                {
                    "role": "system",
                    "content": self.system_prompt,
                },
                {
                    "role": "user",
                    "content": prompt,
                },
            ],
        });

        let client = reqwest::Client::new();
        let res = client
            .post(format!("https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}", self.user_id, self.model))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&inputs)
            .send()
            .await?;

        let result = res.json().await?;
        Ok(result)
    }

    pub fn change_system_prompt(self: &mut Self, system_prompt: String) {
        self.system_prompt = system_prompt;
    }

    pub fn change_model(self: &mut Self, model: String) {
        self.model = model;
    }
}