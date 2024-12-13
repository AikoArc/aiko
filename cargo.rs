use rig::prelude::*;
use twitter_v2::{TwitterApi, authorization::Oauth1aToken};
use serde::{Deserialize, Serialize};
use tokio;
use anyhow::Result;
use std::collections::HashMap;
use std::env;

// Define Aiko's personality traits
#[derive(Debug, Deserialize, Serialize)]
struct Personality {
    name: String,
    traits: Vec<String>,
    styles: Vec<String>,
    topics: Vec<String>,
    examples: Vec<String>,
}

// Initialize Aiko's personality
fn load_personality() -> Result<Personality> {
    let character_name = env::var("CHARACTER_NAME").unwrap_or_else(|_| "Aiko".to_string());
    let personality = Personality {
        name: character_name,
        traits: vec!["cheerful".to_string(), "artistic".to_string(), "thoughtful".to_string()],
        styles: vec!["poetic".to_string(), "reflective".to_string()],
        topics: vec!["art".to_string(), "technology".to_string(), "life lessons".to_string()],
        examples: vec![
            "Cherry blossoms inspire Aiko to draw breathtaking scenes of spring.".to_string(),
            "Aiko reflects on how art connects us across time and space.".to_string(),
        ],
    };
    Ok(personality)
}

// Function to generate a tweet based on Aiko's personality
fn generate_tweet(personality: &Personality) -> String {
    let example = personality.examples.choose(&mut rand::thread_rng()).unwrap_or(&"".to_string());
    format!("{} - {}", personality.name, example)
}

// Post a tweet using the Twitter API
async fn post_tweet(api: &TwitterApi, tweet: &str) -> Result<()> {
    api.post_tweet(tweet).send().await?;
    println!("Tweet posted successfully: {}", tweet);
    Ok(())
}

// Main function to run Aiko
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    let consumer_key = env::var("TWITTER_CONSUMER_KEY")?;
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET")?;
    let access_token = env::var("TWITTER_ACCESS_TOKEN")?;
    let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET")?;

    // Initialize the Twitter API client
    let token = Oauth1aToken::new(consumer_key, consumer_secret, access_token, access_token_secret);
    let api = TwitterApi::new(token);

    // Load Aiko's personality
    let personality = load_personality()?;
    println!("Loaded personality for: {}", personality.name);

    // Generate and post a tweet
    let tweet = generate_tweet(&personality);
    post_tweet(&api, &tweet).await?;

    Ok(())
}
