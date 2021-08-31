use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde::Deserialize;
use reqwest::Result;

#[derive(Deserialize, Debug)]
pub struct RandomFact {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

const JOKE_API: &str = "https://uselessfacts.jsph.pl/random.json?language=en";

#[command("randomFact")]
#[aliases("rf", "rngF")]
pub async fn random_fact(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = query_joke_api().await?;
    msg.channel_id.say(&ctx.http, joke.text).await?;
    
    Ok(())
}

async fn query_joke_api() -> Result<RandomFact> {
    let response = reqwest::get(JOKE_API).await?;
    let random_fact: RandomFact = response.json().await?;

    Ok(random_fact)
}
