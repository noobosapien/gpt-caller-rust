use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Clone)]

pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]

pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Clone)]

pub struct ImgaeGen {
    pub model: String,
    pub prompt: String,
    pub n: u32,
    pub size: String,
}

#[derive(Debug, Deserialize)]

pub struct APIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]

pub struct APIChoice {
    pub message: APIMessage,
}
#[derive(Debug, Deserialize)]

pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}

#[derive(Debug, Deserialize)]

pub struct IMGData {
    pub revised_prompt: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]

pub struct IMGResponse {
    pub data: Vec<IMGData>,
}

#[tokio::main]
async fn main() {
    println!("\nAsk chat gpt 4:\n");
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", &input);

    let message: Message = Message {
        role: "system".to_string(),
        content: input.clone(),
    };

    let response = call_gpt(vec![message.clone()]).await;
    // let response = call_gen_image(input.clone()).await;

    match response {
        Ok(resp) => println!("\n {}", resp),
        Err(e) => {
            println!("{}", &e)
        }
    }
}

async fn call_gen_image(prompt: String) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").expect("Did not find the api key");
    let api_org: String = env::var("OPEN_AI_ORG").expect("Did not find the org key");

    let url: &str = "https://api.openai.com/v1/images/generations";

    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let img_gen: ImgaeGen = ImgaeGen {
        model: "dall-e-3".to_string(),
        prompt,
        n: 1,
        size: "1024x1024".to_string(),
    };

    let res: IMGResponse = client
        .post(url)
        .json(&img_gen)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.data[0].url.clone())
}

async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").expect("Did not find the api key");
    let api_org: String = env::var("OPEN_AI_ORG").expect("Did not find the org key");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4o".to_string(),
        messages,
        temperature: 1.,
    };

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}
