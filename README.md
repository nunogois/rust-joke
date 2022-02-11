# rust-joke

A simple command line joke app written in Rust using https://sv443.net/jokeapi/v2/

Based on my old [go-joke](https://github.com/nunogois/go-joke) project.

There's a specific branch [reference](https://github.com/nunogois/rust-joke/tree/reference) that shows setting the joke variable as a reference, instead of being returned in the Result Ok.

```sh
# Build the app:
cargo build # or cargo build --release if you want to build in release mode

# Run the app:
cargo run

# Run the app with a specific category:
cargo run programming

# Install the app:
cargo install --path . # or a specific path

# Run the installed app:
rust-joke

# Run the installed app with a specific category:
rust-joke programming
```

I'm still learning Rust, so let me know if you catch something wrong!
