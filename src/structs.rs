//! # All return structs used by my functions
//! 
//! This module contains the structs that hold the return objects of the APIs.
//! 

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[cfg(feature = "anime")]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimechanResponse {
    /// A list of all anime titles, only used when [AnimechanRout::ListAllAvailableAnime](AnimechanRout::ListAllAvailableAnime) is used.
    pub anime_list: Option<Vec<String>>,
    /// Anime the quote is from.
    pub anime: Option<String>,
    /// Character that said the quote.
    pub character: Option<String>,
    /// The quote itself.
    pub quote: Option<String>,
    // ? check for errors and return an actual error instead of putting it in this object
    /// If there is an error output by the API this will contain the response.
    /// You should always check if this exists in the first entry of your response.
    pub error: Option<String>,
}

#[cfg(feature = "anime")]
#[derive(Deserialize, Serialize)]
struct AnimeList(pub Vec<String>);

#[cfg(feature = "anime")]
#[derive(Debug, Clone, PartialEq)]
pub enum AnimechanRout {
    /// Get a random quote.
    Random,
    /// Get 10 random quotes.
    Quotes,
    /// Get a list of all available anime to filter by.
    ListAllAvailableAnime,
    /// Search for quotes by anime title. (Outputs a list of 10)
    QuotesByAnime,
    /// Search quotes by character name. No there isn't a list available, I'm sorry! (Outputs a list of 10)
    QuotesByCharacter,
}