use std::env;
use std::collections::HashMap;

use regex::Regex;
use log::debug;
use reqwest::header::{USER_AGENT, COOKIE};

const AOC_USER_AGENT: &str = "Nick's AoC Puzzle Solver <http://github.com/1npo/aoc-rs>";

fn get_session_token() -> String {
    let token = match env::var("AOC_SESSION_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("You must provide a session token to get your puzzle input. \
                          Please put your session token in the AOC_SESSION_TOKEN \
                          environment variable and try again. Quitting."),
    };

    token
}

pub fn get_puzzle_input(
    year: u16,
    day: u8
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::new();
    let res = client.get(url.clone())
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
    let res = client.post(url)
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
        Some(capture) => &capture[0].replace("<article><p>", "").replace(" <a", ""),
        None => "",
    };

    let answer_response = match res_text {
        _ if res_text.contains("That's the right answer") => {
            "Puzzle solved! Great job!"
        },
        _ if res_text.contains("That's not the right answer") => {
            "Wrong answer. Try again!"
        },
        _ if res_text.contains("Did you already complete it") => {
            "Puzzle was already solved. Don't try again!"
        },
        _ if res_text.contains("You gave an answer too recently") => {
            // TODO: Figure out why this doesn't work and fix it.
            //
            // let re: Regex = Regex::new(r"You have (?:(\d+)m )?(\d+)s left to wait")
            //                       .unwrap();
            // let caps = re.captures(text).unwrap;
            // let eta = match caps {
            //     Some(capture) => &capture[0],
            //     None => "",
            // };
            // 
            // eta
            "Rate limit exceeded. Wait a few more minutes before trying again."
        },
        _ => {
            "Got an unexpected response from adventofcode.com."
        },
    };

    Ok(String::from(answer_response))
}
