use anyhow::Ok;
use async_openai::types::chat::ChatCompletionRequestSystemMessageArgs;
use async_openai::types::chat::ChatCompletionRequestUserMessageArgs;
use async_openai::types::chat::CreateChatCompletionRequestArgs;

pub async fn chat_completion(model: &str, system: Option<&str>, prompt: &str) -> anyhow::Result<String>  {
    let client = async_openai::Client::new();
    let mut messages  = vec![];

    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
            .content(system)
            .build()?
            .into()
        );
    }


    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()?
        .into()
    );


    let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .max_tokens(2048u32)
            .build()?;


    let response = client.chat().create(request).await?;

    tracing::info!("response: {:#?}", response);

    Ok("".to_string())
    
}