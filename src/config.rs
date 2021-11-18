use std::{num::NonZeroU32, str::FromStr};

use nonzero_ext::nonzero;

#[derive(Debug)]
pub struct Scraper {
    pub requests_per_second: NonZeroU32,
}

#[derive(Debug)]
pub struct Sqlite {
    pub path: String,
}

#[derive(Debug)]
pub struct Settings {
    pub scraper: Scraper,
    pub sqlite: Sqlite,
}

fn parse<T: FromStr>(name: &'static str) -> Option<T> {
    std::env::var(name).ok().map(|s| s.parse().ok().unwrap())
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            scraper: Scraper {
                requests_per_second: parse("FTS_SCRAPER_REQUESTS_PER_SECOND")
                    .unwrap_or(nonzero!(5u32)),
            },
            sqlite: Sqlite {
                path: std::env::var("FTS_DATABASE_SQLITE_PATH")
                    .unwrap_or_else(|_| "data/folketinget.sqlite3".to_string()),
            },
        }
    }
}
