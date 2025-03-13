use Airi::Airi;

#[tokio::main]
async fn main() {
    let mut assistant = Airi::new(env!("GROQ_API").to_string());
    assistant.text_chat().await;
}