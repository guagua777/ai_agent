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

    let model = "deepseek-v4-flash";

    let response = chat_completion(OpenAI_GPT_OSS, Some("你是一个全能助手"), "中国的首都是哪里").await?;

    println!("response: {}", response);
    
    Ok(())
}
