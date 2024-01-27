use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io;(stdin, stdout, write);

#[derive(Deserialize, Debug)]
struct OAIResponse{
    id: Option<String>,
    object:Option<String>,
    created:Option<u64>,
    model:Option<String>
    choices: Vec<OAIChoices>
}

#[derive(Deserialize, Debug)]
struct  OAIChoices{
    text: string,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive (Serialize, Debug)]
struct OAIRequest{
    prompt: String,
    max_token: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + send + sync>> {
    dotenv().ok();

    let https = HttpConnector::new();
    let client = Client::builder().build(https);
    let url = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Generate a Sql code for the given statement";
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
     
} 