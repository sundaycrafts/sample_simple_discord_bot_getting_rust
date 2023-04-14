use discord_command::Command as DiscordCommand;
use reqwest::header;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
pub struct Choice {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub option_type: u8,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub choices: Vec<Choice>,
}

#[derive(Serialize)]
struct Command {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub command_type: u8,
    pub options: Option<Vec<CommandOption>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = format!(
        "https://discord.com/api/v10/applications/{}/commands",
        std::env::var("APP_ID")?
    );

    let mut header = header::HeaderMap::new();
    header.insert(
        "Authorization",
        format!("Bot {}", std::env::var("DISCORD_TOKEN")?).parse()?,
    );
    header.insert("Content-Type", "application/json; charset=utf-8".parse()?);
    header.insert(
        "User-Agent",
        "DiscordBot (https://github.com/discord/discord-example-app, 1.0.0)".parse()?,
    );

    let client = reqwest::blocking::Client::new();
    let body = vec![Command {
        name: serde_json::to_string(&DiscordCommand::Ask)?,
        description: "Ask a question".into(),
        command_type: 1,
        options: None,
    }];

    let res = client
        .put(endpoint)
        .headers(header)
        .body(serde_json::to_string(&body)?)
        .send()?;

    if res.status().is_success() {
        println!("Success!");
        Ok(())
    } else {
        println!("Something else happened. Status: {:?}", res.status());
        Err("something went wrong".into())
    }
}
