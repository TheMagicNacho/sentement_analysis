mod summarizer;
use summarizer::Summarizer;

use std::env;

#[async_std::main]
async fn main() {
    // Argument parsing
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No file provided.");
        return;
    }

    let summary: String = Summarizer::read_file(&args[1]).await;

    println!("--SUMMARY-- \n {}", summary);
}
