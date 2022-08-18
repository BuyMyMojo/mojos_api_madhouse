[![Crates.io](https://img.shields.io/crates/v/mojos_api_madhouse.svg)](https://crates.io/crates/mojos_api_madhouse)
![Crates.io](https://img.shields.io/crates/l/mojos_api_madhouse)
[![wakatime](https://wakatime.com/badge/user/fd57ff6b-f3f1-4957-b9c6-7e09bc3f0559/project/d5b792fb-32c4-47cd-9d90-dc3c21064f87.svg)](https://wakatime.com/badge/user/fd57ff6b-f3f1-4957-b9c6-7e09bc3f0559/project/d5b792fb-32c4-47cd-9d90-dc3c21064f87)
![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/BuyMyMojo/mojos_api_madhouse/Rust/main)

# mojos_api_madhouse

This is a library I made so I can intergreate more APIs in my discord bot's easily.

it is asynchronous and powered by Reqwest.

My main goal is to make an easy to use library for many things and have features for each type of api so you can have faster compile times/less dead code then what is necessary.

## Examples

```rust
use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse, Error};
use mojos_api_madhouse::anime::animechan;

async fn rand_quote() -> Result<(), Error> {

let output = animechan(AnimechanRout::Random, None, None).await;

let quote: AnimechanResponse = output?.first().unwrap(/* The vec should never return empty */).to_owned();

println!("Your random quote: {}", quote.quote.unwrap_or_else(|| quote.error.unwrap(/* If it gets to this then there should 100% be something in here */)));

Ok(())
}
```
