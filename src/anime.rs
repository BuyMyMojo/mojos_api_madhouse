//! # Anime related APIs
//! 
//! This module contains the functions for apis generally related to Anime/Manga
//! 
//! - animechan
//!     - An API for quickly grabbing quotes from anime. You can search by anime and character or just get random quotes
//! 

use crate::structs::{AnimechanResponse, AnimechanRout, Error};

/// Get anime quotes from <https://animechan.vercel.app>
///
/// # Arguments
///
/// * `rout` - Define rout of API to call.
/// * `search_query` - Query to use when searching by anime or character, otherwise should be None.
/// * `page` - When searching by anime or character get a specific page of results.
/// 
/// # Examples
/// ```
/// use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse, Error};
/// use mojos_api_madhouse::anime::animechan;
/// 
/// async fn rand_quote() -> Result<(), Error> {
/// 
/// let output = animechan(AnimechanRout::Random, None, None).await;
/// 
/// let quote: AnimechanResponse = output?.first().unwrap(/* The vec should never return empty */).to_owned();
/// 
/// println!("Your random quote: {}", quote.quote.unwrap_or_else(|| quote.error.unwrap(/* If it gets to this then there should 100% be something in here */)));
/// 
/// Ok(())
/// }
/// ```
/// 
/// ```
/// use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse, Error};
/// use mojos_api_madhouse::anime::animechan;
///
/// async fn debug_ten_rand_quotes() -> Result<(), Error> {
/// 
/// let output = animechan(AnimechanRout::Quotes, None, None).await;
/// 
/// let quotes: Vec<AnimechanResponse> = output?.to_owned();
/// 
/// println!("List of quotes: {:?}", quotes);
/// 
/// Ok(())
/// }
/// ```
/// 
/// # Error handeling example
/// ```
/// use mojos_api_madhouse::structs::{AnimechanRout, AnimechanResponse, Error};
/// use mojos_api_madhouse::anime::animechan;
/// 
/// async fn error_quote_by_anime() -> Result<(), Error> {
/// 
///     let output = animechan(AnimechanRout::QuotesByAnime, Some("This anime isn't real!!".to_string()), None).await;
/// 
///     let quotes: Vec<AnimechanResponse> = output?.to_owned();
/// 
///     match &quotes.first().unwrap().error {
///         Some(err) => {println!("Error from API: {}", err)},
///         None => {println!("List of quotes from your anime: {:?}", quotes)},
///     }
/// 
/// Ok(())
/// }
/// ```
/// 
#[cfg(feature = "anime")]
pub async fn animechan(
    rout: AnimechanRout,
    search_query: Option<String>,
    page: Option<u32>,
) -> Result<Vec<AnimechanResponse>, Error> {
    let mut query: String = "https://animechan.vercel.app/api/".to_string();

    let output: Option<Vec<AnimechanResponse>> = match rout {
        AnimechanRout::Random => {
            query.push_str("random");
            let body = reqwest::get(query)
                .await?
                .json::<AnimechanResponse>()
                .await?;
            Some(vec![body])
        }
        AnimechanRout::Quotes => {
            query.push_str("quotes");

            let body = reqwest::get(query)
                .await?
                .json::<Vec<AnimechanResponse>>()
                .await?;

            Some(body)
        }
        AnimechanRout::ListAllAvailableAnime => {
            query.push_str("available/anime");

            // ? There is probably a way to use query in here without cloning right?
            let body = reqwest::get(query)
                .await
                .expect("Failed to get request")
                .json::<Vec<String>>()
                .await
                .expect("Failed to parse json");

            // let list: AnimeList = serde_json::from_str(&body)?;

            Some(vec![AnimechanResponse {
                anime_list: Some(body),
                ..Default::default()
            }])
        }
        AnimechanRout::QuotesByAnime => {
            query.push_str("quotes/anime?title=");
            query.push_str(&urlencoding::encode(
                search_query
                    .unwrap_or_else(|| "Jojo's bizarre adventure".to_string())
                    .as_str(),
            ));
            query.push_str(format!("&page={}", page.unwrap_or(1)).as_str());
            let body = reqwest::get(query).await?;
            let body_text = body.text().await?;

            if body_text.starts_with('[') {
                let body_json: Vec<AnimechanResponse> = serde_json::from_str(&body_text)?;
                Some(body_json)
            } else {
                let body_json: AnimechanResponse = serde_json::from_str(&body_text)?;
                Some(vec![body_json])
            }
        }
        AnimechanRout::QuotesByCharacter => {
            query.push_str("quotes/character?name=");
            query.push_str(&urlencoding::encode(
                search_query
                    .unwrap_or_else(|| "Jotaro Kujo".to_string())
                    .as_str(),
            ));
            query.push_str(format!("&page={}", page.unwrap_or(1)).as_str());
            let body = reqwest::get(query).await?;
            let body_text = body.text().await?;

            if body_text.starts_with('[') {
                let body_json: Vec<AnimechanResponse> = serde_json::from_str(&body_text)?;
                Some(body_json)
            } else {
                let body_json: AnimechanResponse = serde_json::from_str(&body_text)?;
                Some(vec![body_json])
            }
        },
    };

    Ok(output.expect("No output from animechan function"))
}