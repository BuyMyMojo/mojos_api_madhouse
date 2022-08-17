use crate::structs::{AnimechanResponse, AnimechanRout};

/// I'm literally taking this from the poise example bot... I truthfully don't understand Result<> or proper error reporting yet.
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Get anime quotes from <https://animechan.vercel.app>
///
/// # Arguments
///
/// * `rout` - Define rout of API to call.
/// * `search_query` - Query to use when searching by anime or character, otherwise should be None.
/// * `page` - When searching by anime or character get a specific page of results.
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
            let body = reqwest::get(query.clone())
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
                &search_query
                    .unwrap_or("Jojo's bizarre adventure".to_string())
                    .as_str(),
            ));
            query.push_str(format!("&page={}", page.unwrap_or(1)).as_str());
            let body = reqwest::get(query).await?;
            let body_text = body.text().await?;

            if body_text.starts_with("[") {
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
                &search_query
                    .unwrap_or("Jotaro Kujo".to_string())
                    .as_str(),
            ));
            query.push_str(format!("&page={}", page.unwrap_or(1)).as_str());
            let body = reqwest::get(query).await?;
            let body_text = body.text().await?;

            if body_text.starts_with("[") {
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