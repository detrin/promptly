use atty::Stream;
use clap::{Arg, Command};
use reqwest::blocking::Client;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::env;
use std::io::{self, Read};

fn main() {
    let matches = Command::new("promptly")
        .arg_required_else_help(true)
        .version("0.1.0")
        .author("Daniel Herman")
        .about("Wrapper for OpenAI's models")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_parser(clap::value_parser!(String))
            .help("Input string"))
        .arg(Arg::new("prompt")
            .short('p')
            .long("prompt")
            .value_parser(clap::value_parser!(String))
            .help("Prompt string"))
        .arg(Arg::new("model")
            .long("model")
            .value_parser(clap::value_parser!(String))
            .default_value("gpt-4o")
            .help("Name of model to use"))
        .arg(Arg::new("token")
            .long("token")
            .value_parser(clap::value_parser!(String))
            .default_value("")
            .help("OpenAI API token"))
        .arg(Arg::new("temperature")
            .short('t')
            .long("temperature")
            .value_parser(clap::value_parser!(f64))
            .default_value("1.0")
            .help("Temperature for generating responses"))
        .arg(Arg::new("max_tokens")
            .short('m')
            .long("max-tokens")
            .value_parser(clap::value_parser!(usize))
            .default_value("1024")
            .help("Max tokens for the response"))
        .arg(Arg::new("top_p")
            .long("top_p")
            .value_parser(clap::value_parser!(f64))
            .default_value("1.0")
            .help("Top-p sampling"))
        .get_matches();

    let mut input_string = matches.get_one::<String>("input").cloned().unwrap_or_default();
    if input_string.is_empty() {
        if atty::isnt(Stream::Stdin) {
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

    let prompt_string = matches.get_one::<String>("prompt").cloned().unwrap_or_default();
    let temperature: f64 = *matches.get_one::<f64>("temperature").expect("Invalid value for temperature");
    let max_tokens: usize = *matches.get_one::<usize>("max_tokens").expect("Invalid value for max tokens");
    let top_p: f64 = *matches.get_one::<f64>("top_p").expect("Invalid value for top_p");

    let json_payload = json!({
        "model": matches.get_one::<String>("model").cloned().unwrap_or_default(),
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

    let mut openai_api_key = matches.get_one::<String>("token").cloned().unwrap_or_default();
    if openai_api_key.is_empty() {
        openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    }
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .body(json_payload.to_string())
        .send()
        .expect("Request failed")
        .text()
        .expect("Failed to read response text");

    let response_json: JsonValue = serde_json::from_str(&response).expect("Failed to parse JSON");
    let response_text = response_json["choices"][0]["message"]["content"].as_str().expect("Invalid response format");

    println!("{}", response_text);
}