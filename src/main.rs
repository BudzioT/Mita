use dotenv;
use std::io::stdin;
use Mita::Mita;

#[tokio::main]
async fn main() {
    let mut mita: Mita = setup();

    // Listen, send the message, get the answer, read it aloud - simple
    loop {
        println!("Enter a prompt > ");
        let mut prompt = String::new();
        stdin().read_line(&mut prompt).expect("Error getting prompt");
        println!("{}", format!("Answer: {}", mita.generate(prompt.as_str()).await.as_str()));
    }
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
