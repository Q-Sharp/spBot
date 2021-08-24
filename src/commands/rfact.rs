use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde::Deserialize;
use reqwest::Result;

#[derive(Deserialize, Debug)]
struct RandomFact {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

#[command]
pub async fn randomFact(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = query_joke_api();
    msg.channel_id.say(&ctx.http, joke.await?.text).await?;

    Ok(())
}

async fn query_joke_api() -> Result<RandomFact> {
    let request_url = "https://uselessfacts.jsph.pl/random.json?language=en";
    let response = reqwest::get(*&request_url).await?;
    let randomFact: RandomFact = response.json().await?;

    Ok(randomFact)
}