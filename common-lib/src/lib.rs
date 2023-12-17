use curl::easy::{Easy2, Handler, WriteError};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

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

pub trait ParseNums<T> {
    fn parse_nums(&self, pattern: char) -> impl Iterator<Item = T>;
}

impl<T: FromStr> ParseNums<T> for &str {
    fn parse_nums(&self, pattern: char) -> impl Iterator<Item = T> {
        self.split(pattern).filter_map(|s| s.parse::<T>().ok())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl Point {
    pub fn manhatten(&self, rhs: &Self) -> usize {
        ((self.x as isize - rhs.x as isize).abs() + (self.y as isize - rhs.y as isize).abs())
            as usize
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

pub struct RowIter<'a, T> {
    data: &'a Vec<Vec<T>>,
    n: usize,
}

impl<'a, T: Copy> Iterator for RowIter<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.data[0].len() {
            let res = self.data.iter().map(|l| l[self.n]).collect();
            self.n += 1;
            Some(res)
        } else {
            None
        }
    }
}

pub trait IntoRowIter<T> {
    fn row_iter(&self) -> RowIter<T>;
}

impl<T> IntoRowIter<T> for Vec<Vec<T>> {
    fn row_iter(&self) -> RowIter<T> {
        RowIter { data: self, n: 0 }
    }
}
