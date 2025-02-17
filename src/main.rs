use std::io::{stdout, Write};

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    },
};
use clap::Parser;
use futures::StreamExt;
mod config;

/// Chat with LLM in your terminal
#[derive(Parser, Debug, Clone)]
#[command(name = "llmc", bin_name = "llmc", version, about)]
struct LlmcArgs {
    /// Your message to LLM
    message: String,
    /// The prompt to use
    #[arg(short, long)]
    prompt: Option<String>,
    /// Continuous mode with a prompt shell
    #[arg(short, long)]
    shell: bool,
}

fn build_client(
    args: LlmcArgs,
    config: config::Config,
) -> (async_openai::Client<OpenAIConfig>, config::Prompt) {
    let prompt = match args.prompt {
        Some(p) => config.prompts.iter().find(|x| x.name == p),
        None => config.prompts.first(),
    };
    let prompt = match prompt {
        Some(p) => p,
        None => {
            eprintln!("prompt not found");
            std::process::exit(1);
        }
    };
    let provider = config.providers.iter().find(|x| x.name == prompt.provider);
    let provider = match provider {
        Some(p) => p,
        None => {
            eprintln!("provider not found");
            std::process::exit(1);
        }
    };
    let config: async_openai::config::OpenAIConfig = provider.clone().into();
    (async_openai::Client::with_config(config), prompt.clone())
}

fn build_request(
    prompt: config::Prompt,
    message: String,
) -> anyhow::Result<CreateChatCompletionRequest> {
    let req = CreateChatCompletionRequestArgs::default()
        .model(&prompt.model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt.message)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(message)
                .build()?
                .into(),
        ])
        .build()?;
    Ok(req)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = LlmcArgs::parse();
    // println!("{:?}", args);
    let config = match config::Config::try_load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error loading config: {:#?}", e);
            std::process::exit(1);
        }
    };
    // println!("{:#?}", config);
    let (client, prompt) = build_client(args.clone(), config.clone());
    if args.shell {
        eprintln!("shell mode not implemented yet");
    } else {
        let req = match build_request(prompt.clone(), args.message) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("error building request: {:#?}", e);
                std::process::exit(1);
            }
        };
        let mut stream = client.chat().create_stream(req).await?;
        let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            write!(lock, "{}", content).unwrap();
                        }
                    });
                }
                Err(err) => {
                    writeln!(lock, "error: {err}").unwrap();
                }
            }
            stdout().flush()?;
        }
    }
    Ok(())
}
