//! # All return structs used by my functions
//! 
//! This module contains the structs that hold the return objects of the APIs.
//! 

use serde_derive::Deserialize;
use serde_derive::Serialize;

/// Basic boxed error handeling.
/// 
/// I'm literally taking this from the poise example bot... I truthfully don't understand Result<> or proper error reporting yet.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

// AnimeChan: start

#[cfg(feature = "anime")]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// API Response
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
    /// 
    /// You should always check if this exists in the first entry of your response.
    pub error: Option<String>,
}

#[cfg(feature = "anime")]
#[derive(Deserialize, Serialize)]
struct AnimeList(pub Vec<String>);

#[cfg(feature = "anime")]
#[derive(Debug, Clone, PartialEq)]
/// API Route
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

// AnimeChan: end

// Owen Wilson Wow: start

#[cfg(feature = "memes")]
#[derive(Debug, Clone, PartialEq)]
/// API Route
pub enum WowRoute {
    /// Get a random wow.
    Random,
    /// Get a specific "wow" by its index in chronological order.
    /// > Does NOT support filters like year or movie.
    Ordered,
    /// Get a list of all movies.
    AllMovies,
    /// Get a list of all directors.
    AllDirectors,
}

#[cfg(feature = "memes")]
#[derive(Debug, Clone, PartialEq)]
/// Method to sort output by.
pub enum SortWow {
    // Don't sort output
    None,
    /// Group random output by movie
    Movie,
    /// Sort output by release date
    ReleaseDate,
    /// Sort output by release year
    Year,
    /// Group random output by director
    Director,
    /// Sort by the wow's wow count!
    /// > Yes that's the scientific term
    NumberCurrentWow,
}

#[cfg(feature = "memes")]
#[derive(Debug, Clone, PartialEq)]
/// Method to sort output by.
pub enum SortDirection {
    Ascending,
    Descending
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Ascending
    }
}

/// How many Wows you want to get
pub enum WowResultCount {
    Empty,
    Is(u32),
}

impl Default for WowResultCount {
    fn default() -> Self {
        WowResultCount::Is(5_u32)
    }
}

/// When there are more than one Wow they are called Wows!
#[cfg(feature = "memes")]
pub type Wows = Vec<Wow>;

/// An instance of a Wow.
#[cfg(feature = "memes")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// API Response
pub struct Wow {
    /// Source movie title
    pub movie: String,
    /// Movie's release year
    pub year: i64,
    /// Movie's release date
    #[serde(rename = "release_date")]
    pub release_date: String,
    /// Movie's director
    pub director: String,
    /// Owen Wilson's character in the movie
    pub character: String,
    /// Length of the movie
    #[serde(rename = "movie_duration")]
    pub movie_duration: String,
    /// Exact timestamp the Wow occurs
    pub timestamp: String,
    /// The full line containing the Wow
    #[serde(rename = "full_line")]
    pub full_line: String,
    /// This is his X Wow in the movie
    #[serde(rename = "current_wow_in_movie")]
    pub current_wow_in_movie: i64,
    /// Total time he Wows in the movie
    #[serde(rename = "total_wows_in_movie")]
    pub total_wows_in_movie: i64,
    /// Link to movie poster
    pub poster: String,
    /// Link to video clip in different resolutions
    pub video: WowVideo,
    /// Link to just the audio of the Wow
    pub audio: String,
}

#[cfg(feature = "memes")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// List of links to different resolution videos
pub struct WowVideo {
    /// Link to Wow in 1080p
    #[serde(rename = "1080p")]
    pub n1080p: String,
    /// Link to Wow in 720p
    #[serde(rename = "720p")]
    pub n720p: String,
    /// Link to Wow in 480p
    #[serde(rename = "480p")]
    pub n480p: String,
    /// Link to Wow in 360p
    #[serde(rename = "360p")]
    pub n360p: String,
}

// Owen Wilson Wow: end
