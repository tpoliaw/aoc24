use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Read as _},
    path::{Path, PathBuf},
    str::FromStr,
};

use dotenvy::dotenv;

pub fn input(day: u8) -> Input {
    Input(day)
}

pub struct Input(u8);

impl Input {
    pub fn string(self) -> String {
        self.as_value()
    }
    /// Read input and convert to a single value
    pub fn as_value<V>(self) -> V
    where
        V: FromStr,
        <V as FromStr>::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        self.file()
            .read_to_string(&mut buf)
            .expect("Failed to read string");
        buf.parse().expect("Input value was not valid")
    }

    /// Read input a line at a time
    pub fn by_line(self) -> impl Iterator<Item = String> {
        BufReader::new(self.file())
            .lines()
            .map(|line| line.expect("Failed to read line"))
    }

    /// Read input, converting each line at a time with the given function
    pub fn map_by_line<T>(self, map: fn(String) -> T) -> impl Iterator<Item = T> {
        self.by_line().map(map)
    }

    fn download_to(&self, target: &Path) {
        println!("Downloading input file for day {}", self.0);
        let src = format!("https://adventofcode.com/2024/day/{}/input", self.0);
        dotenv().unwrap();
        let response = ureq::get(&src)
            .set(
                "Cookie",
                &format!("session={}", env::var("AOC_SESSION").expect("No session")),
            )
            .call()
            .expect("Input request failed");
        fs::write(
            target,
            response.into_string().expect("Couldn't read response"),
        )
        .expect("Couldn't write to input file");
    }

    fn file(&self) -> File {
        let loc = PathBuf::from(format!("input/day{:02}", self.0));
        if !fs::exists(&loc).expect("Failed to exist file") {
            std::fs::create_dir_all("input").unwrap();
            self.download_to(&loc);
        }
        fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(loc)
            .expect("Couldn't open input file")
    }
}
