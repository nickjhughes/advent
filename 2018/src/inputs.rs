use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;

fn get_cookies() -> HashMap<String, String> {
    let file = File::open("cookies.json").expect("Failed to open cookies file");
    let reader = BufReader::new(file);
    let cookies: HashMap<String, String> =
        serde_json::from_reader(reader).expect("Failed to parse cookies file");
    cookies
}

pub fn get_day_input(day: u8) {
    let path = format!("inputs/input{:#02}", day);
    if fs::metadata(&path).is_ok() {
        return;
    }

    let cookies = get_cookies();
    let mut headers = header::HeaderMap::new();
    for (name, value) in &cookies {
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&format!("{}={}", name, value))
                .expect("Session cookie could not be converted to header"),
        );
    }

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build request client");

    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let resp = client.get(url).send().expect("Failed to load input URL");
    if resp.status() == 400 {
        eprintln!("Not logged into Advent of Code - try updating your session cookie");
        panic!();
    }
    let body = resp.text().expect("Failed to get input response body");

    std::fs::write(&path, body).expect("Failed to write input file");
}
