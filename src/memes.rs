//! # Anime related APIs
//! 
//! This module contains the functions for apis generally related to Anime/Manga
//! 
//! - owen_wilson_wow
//!     - Quickly get a wow from Owen Wilson himself!

#[cfg(feature = "anime")]
pub async fn animechan(
    rout: AnimechanRout,
    search_query: Option<String>,
    page: Option<u32>,
) -> Result<Vec<AnimechanResponse>, Error> {

}