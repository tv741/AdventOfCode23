use curl::easy::{Easy2, Handler, WriteError};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> std::result::Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn get_input(day: usize) -> Result<String> {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.url(&format!("https://adventofcode.com/2023/day/{day}/input"))?;
    easy.cookie_file("cookies.txt")?;

    easy.perform()?;

    let code = easy.response_code()?;
    if code != 200 {
        return Err(format!("Response Code: {code}").into());
    }
    let contents = easy.get_ref();

    Ok(String::from_utf8_lossy(&contents.0).to_string())
}

pub fn get_input_cached(day: usize, example: bool) -> Result<String> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let path = if example {
        format!("{root}/input/example{day}.txt")
    } else {
        format!("{root}/input/day{day}.txt")
    };

    if let Ok(mut input) = File::open(&path) {
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        println!("Read input from {path}");
        Ok(buf)
    } else if !example {
        let input = get_input(day)?;
        let mut file = File::create(&path)?;
        file.write_all(input.as_bytes())?;
        println!("Fetched input from adventofcode.com");
        Ok(input)
    } else {
        Err("Example input is only supported for cached input!".into())
    }
}

pub trait ParseNums {
    fn parse_nums(&self) -> impl Iterator<Item = usize>;
}

impl ParseNums for &str {
    fn parse_nums(&self) -> impl Iterator<Item = usize> {
        self.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
    }
}
