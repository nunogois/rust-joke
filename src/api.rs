use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Joke {
    pub setup: String,
    pub delivery: String,
}

pub async fn get_joke(joke_type: &str, joke: &mut Joke) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://v2.jokeapi.dev/joke/{}?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&type=twopart",
        if joke_type != "" { joke_type } else { "programming,miscellaneous,pun,spooky,christmas" }
    );

    *joke = reqwest::get(&url).await?.json::<Joke>().await?;
    Ok(())
}
