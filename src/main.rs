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
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Generate a Sql code for the given statement";
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    loop{
        print(">");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
            println!("");

        let sp = spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into());

        
        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text), 
            max_tokens: 1000,

    };


    let body = Body::from(serde_json::to_vec(&oai_request)?);

    our body and headers that OpenAI requires
    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json") // In requests, (such as POST or PUT), the client tells the server what type of data is being sent.
        .header("Authorization", &auth_header_val) // takes in a key and a value
        .body(body) 
        .unwrap(); 

    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?; // Aggregate the data buffers from a body asynchronously (hence await). Waiting for all the chunks of data to come back and pull a body out of that.

    let json: OAIResponse = serde_json::from_reader(body.reader())?;


    sp.stop();

    println!(""); 

        println!("{}", json.choices[0].text);
    }
    ok(())
} 