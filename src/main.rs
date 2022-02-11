use std::env::args;
use tokio::time::{sleep, Duration};

mod api;

#[tokio::main]
async fn main() {
    let mut joke_type = "";

    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        joke_type = &args[1];
    }

    let mut joke: api::Joke = api::Joke {
        setup: String::from(""),
        delivery: String::from(""),
    };

    match api::get_joke(&joke_type, &mut joke).await {
        Ok(()) => {
            println!("{}", joke.setup);
            sleep(Duration::from_secs(3)).await;
            println!("{}", joke.delivery);
        }
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    };
}
