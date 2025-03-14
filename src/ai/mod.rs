use reqwest::Client;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

// The main source of power!
#[derive(Debug)]
pub struct Ai {
    groq_key: String,
    chat_history: Vec<Message>,
}

impl Ai {
    pub fn new(groq_key: String) -> Ai {
        let starting_history: Message = Message {
            role: "system".to_string(),
            content: "You are a helpful assistant. You are highly specialized in cybersecurity and are ready to help.".to_string(),
        };
        Ai {
            groq_key,
            chat_history: vec![starting_history],
        }
    }

    // Just request an answer from prompt
    pub async fn get_answer(&mut self, prompt: String) -> String {
        // Some consts for nice look
        const URL: &str = "https://api.groq.com/openai/v1/chat/completions";
        const MODEL: &str = "llama3-70b-8192";
        let msg: Message = Message {
            role: "user".to_string(),
            content: prompt,
        };
        self.chat_history.push(msg);

        let body: String = json!({
            "model": MODEL,
            "messages": self.chat_history
        }).to_string();

        // Just request kindly the answer, I love chains of dots
        let response = Client::new()
            .post(URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.groq_key))
            .body(body)
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
            Some(answer) => {
                let answer_str = answer.to_string();
                let answer_msg: Message = Message {
                    role: "assistant".to_string(),
                    content: answer_str,
                };
                self.chat_history.push(answer_msg);

                answer.to_string()
            },
            None => "Couldn't generate message, try again later".to_string(),
        }
    }
}

// Simple message, role - either user or
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// I need to implement token limits and chat history summarization. Seems like 1 word is around 3-4 tokens
// (going with 4 for safety)
// Let's say we set a limit of 7k tokens, then once it was reached, summarize the oldest message to free up 2k tokens
// Editing the first system prompt and adding a summary to it, with needed indication seems like the best idea
// Just remember not to delete the system prompt by mistake ; p