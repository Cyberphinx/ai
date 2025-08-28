use dotenvy::dotenv;
use rig::prelude::*;
use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    // Create ollama client
    let client = providers::ollama::Client::new();

    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("qwen2.5:14b")
        .preamble("You are a comedian here to entertain the user using humour and jokes.")
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt("Entertain me!").await?;

    println!("{response}");

    Ok(())
}
