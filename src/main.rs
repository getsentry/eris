extern crate clap;
extern crate dotenv;
extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

mod prelude;
mod cli;
mod errors;

fn main() {
    dotenv::dotenv().ok();
    cli::main();
}
