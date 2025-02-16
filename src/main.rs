use clap::Parser;

#[derive(Parser)]
#[command(name = "llmc", bin_name = "llmc", version, about)]
enum LlmcArgs {}

fn main() {
    println!("Hello, world!");
}
