#![doc(issue_tracker_base_url = "https://github.com/BuyMyMojo/mojos_api_madhouse/issues/")]

//! This is a library I made so I can intergreate more APIs in my discord bot's easily.
//!
//! it is asynchronous and powered by Reqwest.
//! 
//! My main goal is to make an easy to use library for many things and have features for each type of api so you can have faster compile times/less dead code then what is necessary.
//!
//! # Examples
//! ```
//! use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse, Error};
//! use mojos_api_madhouse::anime::animechan;
//! 
//! async fn rand_quote() -> Result<(), Error> {
//! 
//! let output = animechan(AnimechanRout::Random, None, None).await;
//! 
//! let quote: AnimechanResponse = output?.first().unwrap(/* The vec should never return empty */).to_owned();
//! 
//! println!("Your random quote: {}", quote.quote.unwrap_or_else(|| quote.error.unwrap(/* If it gets to this then there should 100% be something in here */)));
//! 
//! Ok(())
//! }
//! 
//! ```
//! 
//! 

pub mod structs;

#[cfg(feature = "anime")]
pub mod anime;

#[cfg(feature = "memes")]
pub mod memes;

