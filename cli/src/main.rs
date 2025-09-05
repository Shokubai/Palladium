use clap::Parser;

#[derive(Parser)]
#[command(name = "shaipd")]
#[command(about = "AI Model Testing Framwork")]
struct Cli {
    #[arg(short, long)]
    model: String,
    
    #[arg(short, long)]
    tests: String, // Should be YAML!
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    Ok(())
}
