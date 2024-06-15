mod error;

use regex::Regex;
use atty::Stream;
use clap::{Arg, Command, Parser};
use reqwest::blocking::Client;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::env;
use std::io::{self, Read};
#[derive(Parser, Debug)]
#[command(name = "promptly", version = "0.1.0", author = "Daniel Herman", about = "CLI wrapper for LLMs")]
pub struct Args {
    /// Input string
    #[arg(short, long)]
    pub input: Option<String>,
    
    /// Prompt string
    #[arg(short, long)]
    pub prompt: Option<String>,
    
    /// Name of model to use
    #[arg(long, default_value = "gpt-4o")]
    pub model: String,
    
    /// OpenAI API token
    #[arg(long, default_value = "")]
    pub token: String,
    
    /// Temperature for generating responses
    #[arg(short, long, default_value = "1.0")]
    pub temperature: f64,
    
    /// Max tokens for the response
    #[arg(short = 'm', long, default_value = "1024")]
    pub max_tokens: usize,
    
    /// Top-p sampling
    #[arg(long, default_value = "1.0")]
    pub top_p: f64,
    
    /// Extract JSON from response text
    #[arg(long)]
    pub extract_json: bool,
}

fn extract_json_from_response(response_text: &str) {
    let re = Regex::new(r"```json\s*([\S\s]+?)\s*```").unwrap();
    if let Some(caps) = re.captures(response_text) {
        let json_str = caps[1].trim();
        match serde_json::from_str::<JsonValue>(json_str) {
            Ok(parsed_json) => {
                println!("{}", serde_json::to_string_pretty(&parsed_json).unwrap());
            }
            Err(e) => {
                eprintln!("Failed to parse LLM reponse JSON: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("No JSON found in the LLM response");
        std::process::exit(1);
    }
}

fn main() {
    let args = Args::parse();

    let mut input_string = args.input.clone().unwrap_or_default();
    if input_string.is_empty() {
        if atty::isnt(atty::Stream::Stdin) {
            let mut buffer = Vec::new();
            if let Ok(_) = io::stdin().read_to_end(&mut buffer) {
                input_string = String::from_utf8(buffer).unwrap_or_default();
            }
        } else {
            eprintln!("Please provide input using the --input flag or by piping input to the program.");
            std::process::exit(1);
        }
    }

    if input_string.is_empty() {
        eprintln!("Error: Input string is empty. Please provide input using the --input flag or by piping input to the program.");
        std::process::exit(1);
    }

    let prompt_string = args.prompt.clone().unwrap_or_default();
    let temperature: f64 = args.temperature;
    let max_tokens: usize = args.max_tokens;
    let top_p: f64 = args.top_p;
    let extract_json = args.extract_json;

    let json_payload = json!({
        "model": args.model.clone(),
        "messages": [
            {
                "role": "user",
                "content": format!("{} {}", prompt_string, input_string)
            }
        ],
        "temperature": temperature,
        "max_tokens": max_tokens,
        "top_p": top_p,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    let mut openai_api_key = args.token.clone();
    if openai_api_key.is_empty() {
        openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    }
    let client = Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .body(json_payload.to_string())
        .send();
    let response = match response {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Request failed: {}", err);
            std::process::exit(1);
        }
    };
    let response_text = response.text();
    let response_text = match response_text {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Failed to read response text: {}", err);
            std::process::exit(1);
        }
    };

    let response_json: JsonValue = serde_json::from_str(&response_text).expect("Failed to parse JSON");
    // println!("{:#}", response_json);

    if error::ErrorResponse::is_error(&response_text) {
        println!("This API reponse JSON contains an error message.");
        if let Ok(error_response) = error::ErrorResponse::from_json(&response_text) {
            eprintln!("Error code: {}", error_response.error.code);
            eprintln!("Error message: {}", error_response.error.message);
            std::process::exit(1);
        }
    } else {
        let response_text = response_json["choices"][0]["message"]["content"].as_str().expect("Invalid response format");

        if extract_json {
            extract_json_from_response(response_text);
        } else {
            println!("{}", response_text);
        }   
    }
}