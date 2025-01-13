use crate::ai::AI;

mod ai;
mod transcription;

pub struct Mita {
    ai: AI,
}

impl Mita {
    pub fn new(system_prompt: &str, cloudflare_api_key: &str, cloudflare_user_id: &str,
    model: &str) -> Self {
        Self {
            ai: AI::new(
                cloudflare_api_key,
                cloudflare_user_id,
                model,
                system_prompt,
            ),
        }
    }

    pub async fn generate(self: &mut Self, user_prompt: &str) -> String {
        let response = self.ai.generate(user_prompt).await.expect("Couldn't generate AI's response");
        if response["success"] == false {
            String::from("Couldn't retrieve the answer")
        }
        else {
            response["result"]["response"].to_string()
        }
    }
}