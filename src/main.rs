mod commands;

use std::{collections::HashSet, env, sync::Arc};

use commands::{rfact::*, meta::*, owner::*};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(random_fact, ping, quit)]
struct General;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    let token = match Some(String::from(&args[0])) {
        Some(f) => f,
        None => env::var("DISCORD_TOKEN").expect("Expected a token in the environment")
    };

    let prefix = match Some(String::from(&args[1])) {
        Some(f) => f,
        None => env::var("DISCORD_PREFIX").expect("Expected a prefix in the environment")
    };

    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix(&prefix)).group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
