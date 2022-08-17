# mojos_api_madhouse

This is a library I made so I can intergreate more APIs in my discord bot's easily.

it is asynchronous and powered by Reqwest.

My main goal is to make an easy to use library for many things and have features for each type of api so you can have faster compile times/less dead code then what is necessary.

## Examples
```rust
use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse};
use mojos_api_madhouse::anime::animechan;

let output = animechan(AnimechanRout::Random, None, None).await?;

let quote: AnimechanResponse = output.except("No response from AnimeChan API").first().expect("There should always be an output!").to_owned();

println!("Your random quote: {}", quote.quote.unwrap_or(error.error.unwrap()));

```

License: Apache-2.0
