use serde_json::{json, Value};
use anyhow::Result;
use reqwest;


// Literally AI, just AI wrapper...
pub struct AI {
    api_token: String,
    user_id: String,
    model: String,
    system_prompt: String,
}

impl AI {
    pub fn new(api_token: &str, user_id: &str, model: &str, system_prompt: &str) -> Self {
        Self {
            api_token: api_token.to_string(),
            user_id: user_id.to_string(),
            model: model.to_string(),
            system_prompt: system_prompt.to_string(),
        }
    }

    // Just try to get an answer from cloudflare Llama by HTTP request
    pub async fn generate(self: &mut Self, prompt: &str) -> Result<Value> {
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

    pub fn change_system_prompt(self: &mut Self, system_prompt: &str) {
        self.system_prompt = system_prompt.to_string();
    }
}