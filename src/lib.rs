#![doc(issue_tracker_base_url = "https://github.com/BuyMyMojo/mojos_api_madhouse/issues/")]

//! This is a library I made so I can intergreate more APIs in my discord bot's easily.
//!
//! it is asynchronous and powered by Reqwest.
//! 
//! My main goal is to make an easy to use library for many things and have features for each type of api so you can have faster compile times/less dead code then what is necessary.
//!
//! # Examples
//! ```
//! use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse};
//! use mojos_api_madhouse::anime::animechan;
//! 
//! async fn rand_quote() {
//! 
//! let output = animechan(AnimechanRout::Random, None, None).await;
//! 
//! let quote: AnimechanResponse = output.expect("No response from AnimeChan API").first().expect("There should always be an output!").to_owned();
//! 
//! println!("Your random quote: {}", quote.quote.unwrap_or(quote.error.unwrap()));
//! 
//! }
//! 
//! ```

pub mod structs;

#[cfg(feature = "anime")]
pub mod anime;

