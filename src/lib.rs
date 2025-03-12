use crate::ai::AI;
use crate::transcription::Transcription;

mod ai;
mod transcription;

// Your helpful as heck AI waifu, enjoy!
pub struct Airi {
    ai: AI,
    transcription: Transcription,
}

impl Airi {
    // Create a new Airi, unlimited waifu production...
    pub fn new(system_prompt: &str, cloudflare_api_key: &str, cloudflare_user_id: &str,
    ai_model: &str, vosk_model: &str) -> Self {
        Self {
            ai: AI::new(
                cloudflare_api_key,
                cloudflare_user_id,
                ai_model,
                system_prompt,
            ),
            transcription: Transcription::new(
                vosk_model
            )
        }
    }

    // Just poke her about an answer to your message
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