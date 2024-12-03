use std::env;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use regex::Regex;
use log::{info, debug};
use reqwest::header::{USER_AGENT, COOKIE};
use homedir::my_home;

const AOC_USER_AGENT: &str = "aoc-rs-1npo <http://github.com/1npo/aoc-rs>";

/// Get the daily puzzle input from a cache file, if it exists, otherwise GET it from the
/// AoC website and cache it to a file -- to avoid making unnecessary requests. Stores
/// cache files in:
///
/// 1. `XDG_CACHE_DIR` environment variable, if set, otherwise:
/// 2. `~/.cache/aoc-rs/`, if `~` is resolved by `homedir::my_home()`, otherwise:
/// 3. `/tmp/.cache/.aoc-rs/`
pub fn get_input(
    year: u16,
    day: u8
) -> Result<String, Box<dyn std::error::Error>> {
    let filename = format!("aoc_input_{year}_{day}");
    let mut p = match env::var("XDG_CACHE_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            let mut p = match my_home()? {
                Some(home) => PathBuf::from(home),
                None => PathBuf::from("/tmp"),
            };
            p.push(".cache");
            p
        }
    };

    p.push("aoc-rs");
    p.push(Path::new(&filename).file_stem().unwrap());
    p.set_extension("txt");

    if !p.exists() {
        info!("Puzzle input not cached. Downloading...");
        match get_puzzle_input(year, day) {
            Ok(input) => {
                let cache_dir_prefix = p.parent().unwrap();
                std::fs::create_dir_all(cache_dir_prefix).unwrap();            
                std::fs::write(p, &input).unwrap();
                return Ok(input)
            },
            Err(e) => return Err(e)
        }
    }

    debug!("Cached to file {p:?}");
    info!("Got puzzle input from cached file");

    Ok(std::fs::read_to_string(p).unwrap())
}

fn get_session_token() -> String {
    let token = match env::var("AOC_SESSION_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!(
            "You must provide a session token to get your puzzle input. Please put your \
            session token in the AOC_SESSION_TOKEN environment variable and try again. \
            Quitting.")
    };

    token
}

pub fn get_puzzle_input(
    year: u16,
    day: u8
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url.clone())
        .header(USER_AGENT, AOC_USER_AGENT)
        .header(COOKIE, get_session_token())
        .send()?;

    debug!("Got response: {:?} {}", res.version(), res.status());
    debug!("Response headers: {:?}", res.headers());

    Ok(res.text()?)
}

pub fn post_puzzle_answer(
    year: u16,
    day: u8,
    part: u8,
    answer: String
) -> Result<String, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("level", part.to_string());
    params.insert("answer", answer);

    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url)
        .header(USER_AGENT, AOC_USER_AGENT)
        .header(COOKIE, get_session_token())
        .form(&params)
        .send()?;
    
    debug!("Got response: {:?} {}", res.version(), res.status());
    debug!("Response headers: {:?}", res.headers());

    let res_raw_text = res.text()?;
    let re: Regex = Regex::new(r"<article><p>(?<result>.*)<a").unwrap();
    let caps = re.captures(res_raw_text.as_str());
    let res_text = match caps {
        Some(capture) => String::from(&capture[0])
            .replace("<article><p>", "")
            .replace(" <a", ""),
        None => String::from(""),
    };

    let answer_response = match res_text {
        _ if res_text.contains("That's the right answer") => {
            String::from("Puzzle solved! Great job!")
        },
        _ if res_text.contains("That's not the right answer") => {
            String::from("Wrong answer. Try again!")
        },
        _ if res_text.contains("Did you already complete it") => {
            String::from("Puzzle was already solved. Don't try again!")
        },
        text if res_text.contains("You gave an answer too recently") => {
            let re: Regex = Regex::new(r"You have (?:(\d+)m )?(\d+)s left to wait")
                .unwrap();
            let caps = re.captures(text.as_str());
            let eta = match caps {
                Some(capture) => format!(
                    "Rate limit exceeded. Please wait another {}m {}s before trying \
                    again.",
                    String::from(&capture[1]),
                    String::from(&capture[2])),

                None => String::from(""),
            };
            
            eta
        },
        _ => {
            String::from("Got an unexpected response from adventofcode.com.")
        },
    };

    Ok(answer_response)
}
