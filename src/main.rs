use anyhow::Ok;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

use crate::llm::complete::chat_completion;
use crate::constant::NVIDIA_Nemotron_3;
use crate::constant::OpenAI_GPT_OSS;

mod llm;
mod constant;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let url = std::env::var("OPENAI_BASE_URL")
                    .expect("OPENAI_BASE_URL environment variable not set");
    println!("OPENAI_BASE_URL: {}", url);            

    let subscriber = FmtSubscriber::builder()
                    .with_max_level(Level::INFO)
                    .finish();
                
    tracing::subscriber::set_global_default(subscriber)?;

    let _response = chat_completion("gpt-3.5-turbo", Some("你是一个全能助手"), "中国的首都是哪里").await?;

    Ok(())
}
