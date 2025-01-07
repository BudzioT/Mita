use dotenv;
use std::io::stdin;

mod ai;


#[tokio::main]
async fn main() {
    let mut ai = setup();

    loop {
        println!("Enter a prompt > ");
        let mut prompt = String::new();
        stdin().read_line(&mut prompt).expect("Error getting prompt");
        println!("{}", format!("Answer: {}", ai.generate(prompt).await.expect("Error getting answer")));
    }
}

fn setup() -> ai::AI {
    let system_prompt = "You are ExploreAI, an outdoorsman’s expert assistant. Keep answers short.";

    let ai = ai::AI::new(
        dotenv::var("CF_API_KEY").unwrap(),
        dotenv::var("CF_USER_ID").unwrap(),
        String::from("@cf/meta/llama-3-8b-instruct"),
        String::from(system_prompt),
    );

    ai
}
