use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{Body, Client, Request, header};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{Write, stdin, stdout};

#[derive(Serialize, Deserialize, Debug)]
struct OpenAiChoices {
    index: u8,
    message: OpenAiResponseMessage,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAiResponseMessage {
    role: String,
    content: String,
    refusal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAiResponse {
    id: String,
    object: String,
    created: u32,
    model: Option<String>,
    choices: Vec<OpenAiChoices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAiContent {
    _type: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    max_completion_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/chat/completions";
    let preamble = "Answer the following question accurately.";
    let open_ai_token: String = env::var("OPEN_AI_TOKEN").expect("OPEN_AI_TOKEN must be set");
    let auth_header_val = format!("Bearer {}", open_ai_token);

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut user_content = String::new();

        stdin()
            .read_line(&mut user_content)
            .expect("Failed to read line");

        if user_content.trim() == "exit" {
            break;
        }

        println!("");

        let sp = Spinner::new(&Spinners::Dots9, "\t\tOpenAI is Thinking".into());

        let open_ai_request = OpenAiRequest {
            model: format!("{}", "gpt-4o"),
            messages: vec![
                OpenAiMessage {
                    role: format!("{}", "system"),
                    content: format!("{}", preamble),
                },
                OpenAiMessage {
                    role: format!("{}", "user"),
                    content: format!("{}", user_content),
                },
            ],
            max_completion_tokens: 100,
        };

        let body = Body::from(serde_json::to_vec(&open_ai_request)?);

        let request = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

        let response = client.request(request).await?;

        let body = hyper::body::aggregate(response).await?;

        let json: OpenAiResponse = serde_json::from_reader(body.reader())?;

        sp.stop();

        println!("");

        println!("{}", json.choices[0].message.content);
    }

    Ok(())
}
