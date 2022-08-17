#[cfg(test)]
mod tests {
    use mojos_api_madhouse::{animechan, AnimechanRout};

    #[test]
    fn tests_can_at_least_run() {
        assert_eq!(4, 4);
    }

    /// A quick macro to help test async functions. Taken from: https://blog.x5ff.xyz/blog/async-tests-tokio-rust/
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_random() {
        let res = aw!(animechan(AnimechanRout::Random, None, None));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }
    
    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quotes() {
        let res = aw!(animechan(AnimechanRout::Quotes, None, None));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_anime_list() {
        let res = aw!(animechan(AnimechanRout::ListAllAvailableAnime, None, None));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quote_by_anime_none() {
        let res = aw!(animechan(AnimechanRout::QuotesByAnime, None, None));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quote_by_anime() {
        let res = aw!(animechan(
            AnimechanRout::QuotesByAnime,
            Some("Ghost in the shell".to_string()),
            None
        ));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quote_by_anime_error() {
        let res = aw!(animechan(
            AnimechanRout::QuotesByAnime,
            Some("Ghost in the shell season 69".to_string()),
            None
        ));
        assert_eq!(
            true,
            res.unwrap().first().expect("Result vec empty!").error.is_some()
        );
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quote_by_character_none() {
        let res = aw!(animechan(AnimechanRout::QuotesByCharacter, None, None));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quote_by_character() {
        let res = aw!(animechan(
            AnimechanRout::QuotesByCharacter,
            Some("Edward Elric".to_string()),
            None
        ));
        assert_eq!(None, res.unwrap().first().expect("Result vec empty!").error);
    }

    #[test]
    #[cfg(feature = "anime")]
    fn animechan_quote_by_character_error() {
        let res = aw!(animechan(
            AnimechanRout::QuotesByCharacter,
            Some("Lord Vader".to_string()),
            None
        ));
        assert_eq!(
            true,
            res.unwrap().first().expect("Result vec empty!").error.is_some()
        );
    }
}
