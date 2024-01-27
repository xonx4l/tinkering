use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io;(stdin, stdout, write);

#[derive(Deseralize, Debug)]
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
