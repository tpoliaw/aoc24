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

    pub fn by_line(self) -> impl Iterator<Item = String> {
        BufReader::new(self.file())
            .lines()
            .map(|line| line.expect("Failed to read line"))
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

    fn path(&self) -> PathBuf {
        PathBuf::from(format!("input/day{:02}", self.0))
    }

    fn file(&self) -> File {
        let loc = self.path();
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
