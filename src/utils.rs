use crate::giftexchange::ExchangePool;
use chrono::{Datelike, Local};

/// Returns a letter identifier for the given exchange pool
///
/// - IslandLife always returns 'I'
/// - Pets always returns 'P'
/// - Grabergishimazureson returns a random letter from a predefined set
pub fn letter_for_pool(pool: ExchangePool) -> char {
    let letters = "ACDIJLMNORSTUXYZ".chars().collect::<Vec<char>>();

    match pool {
        ExchangePool::IslandLife => 'I',
        ExchangePool::Grabergishimazureson => fastrand::choice(letters.iter()).unwrap().clone(),
        ExchangePool::Pets => 'P',
    }
}

/// Returns the current year
pub fn current_year() -> i32 {
    Local::now().year()
}

/// Parses command line pool argument into ExchangePool enum
pub fn parse_pool_arg(arg: &str) -> Result<ExchangePool, String> {
    match arg {
        "island" => Ok(ExchangePool::IslandLife),
        "graber" => Ok(ExchangePool::Grabergishimazureson),
        "pets" => Ok(ExchangePool::Pets),
        _ => Err(format!("Invalid pool specified: '{}'. Valid options are: island, graber, pets", arg)),
    }
}
