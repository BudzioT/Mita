use reqwest::Client;
use serde_json::{json, Value};


// The main source of power!
pub struct Ai {
    groq_key: String,
}

impl Ai {
    pub fn new(groq_key: String) -> Ai {
        Ai {
            groq_key,
        }
    }

    // Just request an answer from prompt
    pub async fn get_answer(&mut self, prompt: String) -> String {
        // Some consts for nice look
        const URL: &str = "https://api.groq.com/openai/v1/chat/completions";
        const MODEL: &str = "llama3-70b-8192";
        let body: String = json!({
            "model": MODEL,
            "messages": [
                {
                    "role": "system",
                    "content": "You are an helpful AI assistant who knows everything about cyber security"
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }).to_string();

        // Just request kindly the answer, I love chains of dots
        let response = Client::new()
            .get(URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.groq_key))
            .body(body.to_string())
            .send()
            .await;
        if response.is_err() {
            return format!("There was an error generating your message, err: {}", response.err().unwrap());
        }

        // Get response, first choice, and content of the message from it - turn it into string and check if it exists, return it
        let response_text: String = response.unwrap().text().await.unwrap();
        let json: Value = serde_json::from_str(&response_text).unwrap();
        let content = json["choices"][0]["message"]["content"].as_str();
        match content {
            Some(answer) => answer.to_string(),
            None => "Couldn't generate message, try again later".to_string(),
        }
    }
}