use dotenvy::dotenv;
use rig::{cli_chatbot::cli_chatbot, prelude::*};
use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    // Create ollama client
    let client = providers::ollama::Client::new();

    // Create agent with a single context prompt
    let agent = client
        .agent("qwen2.5:14b")
        .preamble("You are a super intelligent robot trying to achieve tehcnical perfection in providing help to coding challenges.")
        .build();

    // Prompt the agent and print the response
    // let response = comedian_agent.prompt("Entertain me!").await?;

    // println!("{response}");

    // Launch the interactive CLI chatbot REPL with the agent
    cli_chatbot(agent).await?;

    Ok(())
}
