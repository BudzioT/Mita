mod ai;

use std::io::{stdin, stdout, Write};
use ai::Ai;

pub struct Airi {
    ai: Ai,
}

impl Airi {
    pub fn new(groq_api_key: String) -> Airi {
        Airi {
            ai: Ai::new(groq_api_key),
        }
    }

    pub async fn text_chat(&mut self) {
        loop {
            print!("> ");
            stdout().flush().unwrap();

            let mut text: String = String::new();
            stdin().read_line(&mut text).expect("Couldn't read the prompt");
            let answer = self.ai.get_answer(text).await;
            println!("{}", answer);
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
