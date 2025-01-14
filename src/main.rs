use dotenv;
use std::io::stdin;
use Mita::Mita;

#[tokio::main]
async fn main() {
    let mut mita: Mita = setup();

    loop {
        println!("Enter a prompt > ");
        let mut prompt = String::new();
        stdin().read_line(&mut prompt).expect("Error getting prompt");
        println!("{}", format!("Answer: {}", mita.generate(prompt.as_str()).await.as_str()));
    }
}

fn setup() -> Mita {
    let system_prompt = "You are ExploreAI, an outdoorsman’s expert assistant. Keep answers short.";

    let mita = Mita::new(
        system_prompt,
        dotenv::var("CF_API_KEY").expect("Cloudlfare API key not found").as_str(),
        dotenv::var("CF_USER_ID").expect("Cloudflare user ID not found").as_str(),
        "@cf/meta/llama-3-8b-instruct",
        "../../data/models/vosk-model-en-us-0.22",
    );

    mita
}
