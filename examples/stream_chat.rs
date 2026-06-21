use anyhow::Result;
use tokio::task::JoinSet;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;
use ai_agent::llm::stream::chat_stream_with_retry;
use ai_agent::llm::semaphore::get_semaphores;
use ai_agent::llm::semaphore::get_provider;
use tracing::Instrument;



#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let url = std::env::var("OPENAI_BASE_URL")
                    .expect("OPENAI_BASE_URL environment variable not set");
    println!("OPENAI_BASE_URL: {}", url);            

    let subscriber = FmtSubscriber::builder()
                    .with_max_level(Level::INFO)
                    .finish();
                
    tracing::subscriber::set_global_default(subscriber)?;

    let model = "deepseek-v4-flash";

    // let s = chat_stream(model, 
    //     Some("你是一个全能助手"), 
    //     "道德经的第四章是什么？");


    // futures::pin_mut!(s);
    // let mut output: String = String::new();
    // while let Some(result) = s.next().await {
    //     match result {
    //         Ok(txt) => {
    //             output.push_str(&txt);
    //             print!("{txt}");
    //         }
    //         Err(err) => {
    //             tracing::error!("\nError while streaming: {}", err);
    //             return Err(err);
    //         }
    //     }
    // }    

    // println!("");
    // println!("response: {:#?}", output);


    let prompts = vec![
        "Do you know the current time in New York",
        "金刚经的第4句是什么？",
        "库索拉是一个国家吗？首都是哪里",
        "What is the difference between Rust and C++?",
        "道德经第一章原文是什么？",
        "Is the moon bigger than the sun?",
        "用一句话解释量子纠缠",
        "Who wrote the Rust programming language?",
        "北京和上海哪个城市更大？",
        "What is the capital of Australia?",
        "解释一下TCP和UDP的区别",
        "How many planets are in the solar system?",
    ];


    let mut set = JoinSet::new();

    for prompt in prompts {
        let span = tracing::info_span!("chat", prompt = prompt);
        set.spawn(
            async move {
                tracing::info!("\n\n{prompt}");
                let permit = get_semaphores()["openai"].acquire().await?;
                let output = chat_stream_with_retry(model, Some("你是一个全能助手"), prompt).await?;
                // 为什么还需要单独drop一下？
                drop(permit);
                Ok::<_, anyhow::Error>((prompt, output))
            }
            .instrument(span),
        );
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt, result))) => tracing::info!("\n{prompt}\n{result}"),
            Ok(Err(err)) => tracing::error!("Task panicked: {err}"),
            Err(err) => tracing::error!("Task panicked: {err}"),
        }
    }

  
    
    Ok(())
}
