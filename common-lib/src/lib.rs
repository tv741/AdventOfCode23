use curl::easy::{Easy2, Handler, WriteError};
use std::fs::File;
use std::io::{Read, Write};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn get_input(day: usize) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.url(&format!("https://adventofcode.com/2023/day/{day}/input"))
        .unwrap();
    easy.cookie_file("cookies.txt").unwrap();

    easy.perform().unwrap();

    assert_eq!(easy.response_code().unwrap(), 200);
    let contents = easy.get_ref();

    String::from_utf8_lossy(&contents.0).to_string()
}

pub fn get_input_cached(day: usize, example: bool) -> String {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let path = if example {
        format!("{root}/input/example{day}.txt")
    } else {
        format!("{root}/input/day{day}.txt")
    };

    if let Ok(mut input) = File::open(&path) {
        let mut buf = String::new();
        input.read_to_string(&mut buf).unwrap();
        println!("Read input from {path}");
        buf
    } else if !example {
        let input = get_input(day);
        let mut file = File::create(&path).unwrap();
        file.write_all(input.as_bytes()).unwrap();
        println!("Fetched input from adventofcode.com");
        input
    } else {
        panic!("Example input is only supported for cached input!");
    }
}
