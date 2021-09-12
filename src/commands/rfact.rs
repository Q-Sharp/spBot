use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde::Deserialize;
use reqwest::Result;
use rand::Rng;

#[derive(Deserialize, Debug)]
pub struct RandomFact {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

impl Fact for RandomFact {
    fn get_fact(&self) -> String {
        format!("{}", self.text)
    }
}

pub trait Fact {
    fn get_fact(&self) -> String;
}

const JOKE_APIS: &'static [&str] = &["https://uselessfacts.jsph.pl/random.json?language=en"];

#[command("randomFact")]
#[aliases("rf", "rngF")]
pub async fn random_fact(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = query_joke_api().await?;
    msg.channel_id.say(&ctx.http, joke.get_fact()).await?;
    
    Ok(())
}

async fn query_joke_api() -> Result<impl Fact> {
    let response = reqwest::get(get_jokeapi_url()).await?;
    let random_fact: RandomFact = response.json().await?;
    Ok(random_fact)
}

fn get_jokeapi_url() -> &'static str {
    JOKE_APIS[rand::thread_rng().gen_range(0..JOKE_APIS.len())]
}
