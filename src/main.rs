use std::sync::Arc;

use langchain_rust::{
    agent::{AgentExecutor, ConversationalAgentBuilder},
    chain::{options::ChainCallOptions, Chain},
    llm::ollama::client::Ollama,
    memory::SimpleMemory,
    prompt_args,
    tools::CommandExecutor,
};

use start_langchain_rust::tools::application_info;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default().with_model("llama3.2:latest");
    let memory = SimpleMemory::new();

    let agent = ConversationalAgentBuilder::new()
        .tools(&[
            Arc::new(CommandExecutor::default()),
            Arc::new(application_info::ApplicationInfo {}),
        ])
        .options(ChainCallOptions::new().with_max_tokens(1000))
        .build(ollama)
        .unwrap();

    let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());

    let input_variables = prompt_args! {
        "input" => "What is current dir: pwd",
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
        "input" => "Execute command 'ls' and show raw output without formatting",
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

    let application_info = prompt_args! {
        "input" => "Show me the raw output from ApplicationInfo tool without any formatting or interpretation",
    };

    println!("Requesting application info...");
    match executor.invoke(application_info).await {
        Ok(b) => {
            println!("\nApplication Info:\n{}", b);
        }
        Err(e) => {
            println!("Error getting app info -> {:?}", e);
        }
    }
}
