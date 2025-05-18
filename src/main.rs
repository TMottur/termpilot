use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

use dirs::config_dir;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Config {
    openai_api_key: String,
}

fn get_config_path() -> PathBuf {
    config_dir()
        .expect("Could not find config directory")
        .join("termpilot")
        .join("config.toml")
}

fn load_or_create_config(force_reset: bool) -> Result<Config, Box<dyn std::error::Error>> {
    let path = get_config_path();

    if force_reset && path.exists() {
        fs::remove_file(&path)?;
        println!("🔄 API key reset. Re-entering...");
    }

    if path.exists() {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        println!("🔑 Enter your OpenAI API key:");
        let mut key = String::new();
        io::stdin().read_line(&mut key)?;
        let key = key.trim().to_string();

        let config = Config {
            openai_api_key: key.clone(),
        };

        let dir = path.parent().unwrap();
        fs::create_dir_all(dir)?;
        let mut file = File::create(path)?;
        file.write_all(toml::to_string(&config)?.as_bytes())?;

        Ok(config)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Handle --help and --version
    if args.contains(&"--help".to_string()) {
        println!("TermPilot — explain terminal output using GPT-4\n\nUsage:\n  explain < input.txt\n  explain --file <file>\n  explain --reset-key\n");
        return Ok(());
    }

    if args.contains(&"--version".to_string()) {
        println!("TermPilot 0.1.0");
        return Ok(());
    }

    let force_reset = args.contains(&"--reset-key".to_string());
    let config = load_or_create_config(force_reset)?;

    let input = if args.len() > 2 && args[1] == "--file" {
        fs::read_to_string(&args[2])?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let prompt = format!(
        "Provide a concise explanation of the following terminal output:\n\n{}",
        input
    );

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(config.openai_api_key)
        .json(&json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()?;

    let json: serde_json::Value = response.json()?;
    let reply = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("⚠️ GPT returned no message.");

    println!("\n🧠 GPT says:\n\n{}\n", reply);

    Ok(())
}
