use core::panic;
use std::sync::Arc;

use langchain_rust::{
    agent::{AgentExecutor, ConversationalAgentBuilder},
    chain::{options::ChainCallOptions, Chain},
    llm::ollama::client::Ollama,
    memory::SimpleMemory,
    prompt_args,
    tools::CommandExecutor,
};

#[tokio::main]
async fn main() {
    let ollama = Ollama::default().with_model("llama3.2:latest");
    let memory = SimpleMemory::new();

    let agent = ConversationalAgentBuilder::new()
        .tools(&[Arc::new(CommandExecutor::default())])
        .options(ChainCallOptions::new().with_max_tokens(1000))
        .build(ollama)
        .unwrap();

    let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());

    let input_variables = prompt_args! {
        "input" => "Please execute: pwd",
    };

    println!("Sending request to agent...");
    let dir_response = executor.invoke(input_variables).await;

    let current_dir: String = match dir_response {
        Ok(v) => {
            println!("\nCurrent directory:\n{}", v);
            v
        }
        Err(e) => {
            println!("error -> {:?}", e);
            "".to_string()
        }
    };
    let list_files_input = prompt_args! {
        "input" => format!("list files in directory {}", current_dir),
    };

    println!("\nListing files...");
    match executor.invoke(list_files_input).await {
        Ok(files) => {
            println!("\nFiles in directory:\n{}", files);
        }
        Err(e) => {
            println!("Error listing files -> {:?}", e);
        }
    }
}
