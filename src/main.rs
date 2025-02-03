use dotenv;
use std::time::Duration;
use cpal::traits::StreamTrait;
use Mita::Mita;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mita: Mita = setup();

    let stream = mita.transcription.start_stream();

    stream.play().expect("Couldn't play the stream");

    println!("Listening for audio input... (will run for 30 seconds)");
    tokio::time::sleep(Duration::from_secs(30)).await;
    println!("Test completed.");
    Ok(())

    /*
    // Listen, send the message, get the answer, read it aloud - simple
    loop {
        println!("Enter a prompt > ");
        let mut prompt = String::new();
        stdin().read_line(&mut prompt).expect("Error getting prompt");
        println!("{}", format!("Answer: {}", mita.generate(prompt.as_str()).await.as_str()));
    }

     */
}

fn setup() -> Mita {
    let system_prompt = "You are ExploreAI, an outdoorsman’s expert assistant. Keep answers short.";

    // Hardcoding the paths cause why not, change it if you want...
    let mita = Mita::new(
        system_prompt,
        dotenv::var("CF_API_KEY").expect("Cloudlfare API key not found").as_str(),
        dotenv::var("CF_USER_ID").expect("Cloudflare user ID not found").as_str(),
        "@cf/meta/llama-3-8b-instruct",
        "C:/Users/Bartosz/RustProjects/Mita/data/models/vosk-model-en-us-0.22",
    );

    mita
}
