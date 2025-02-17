use clap::Parser;

/// Chat with LLM in your terminal
#[derive(Parser, Debug, Clone)]
#[command(name = "llmc", bin_name = "llmc", version, about)]
struct LlmcArgs {
    /// Your message to LLM
    message: String,
    /// Upload the file to LLM
    #[arg(short, long)]
    file: Option<String>,
    /// The profile/model to use
    #[arg(short, long)]
    model: Option<String>,
    /// The prompt to use
    #[arg(short, long)]
    prompt: Option<String>,
    /// Continuous mode with a prompt shell
    #[arg(short, long)]
    shell: bool,
}

#[tokio::main]
async fn main() {
    let args = LlmcArgs::parse();
    println!("{:?}", args);
}
