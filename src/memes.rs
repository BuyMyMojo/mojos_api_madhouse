//! # Meme related APIs
//! 
//! This module contains the functions for apis generally related to Anime/Manga
//! 
//! - owen_wilson_wow
//!     - Quickly get a wow from Owen Wilson himself!

use crate::structs::{WowRoute, Wows, Wow, WowVideo, WowResultCount, SortWow, SortDirection, Error};

#[cfg(feature = "anime")]
pub async fn owen_wilson_wow(
    rout: WowRoute,
    search_query: Option<String>,
    result_count: WowResultCount,
    result_sorting: SortWow,
    sort_direction: SortDirection
) -> Result<Wows, Error> {

    Ok(unimplemented!())
}