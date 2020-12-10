//! Opern Inplementation
//! 

use super::tone::Tones;
use std::{
    fs::{self, File},
    io::BufReader,
};
use rodio::{
    source::Buffered,
    Decoder,
};

pub struct Opern {
    path: String,
    beats: Vec<Option<Buffered<Decoder<BufReader<File>>>>>,
    ptr: usize,
    counter: usize,
}

impl Opern {
    pub fn new(path: &str, tones: &Tones) -> Opern {
        let context = fs::read_to_string(path).unwrap();
        let words: Vec<_> = context.split(' ').collect();
        let mut beats = Vec::new();
        for word in words {
            let word = word.to_string();
            match word.as_str() {
                "#" => {
                    beats.push(None);
                },
                _ => {
                    let beat = tones.get_tone_source(word.as_str()).unwrap();
                    beats.push(Some(beat));
                }
            }
        }
        Self {
            path: String::from(path),
            beats,
            ptr: 0,
            counter: 0,
        }
    }

    pub fn count(&mut self) -> Option<Buffered<Decoder<BufReader<File>>>> {
        if self.ptr == self.beats.len() {
            self.ptr = 0;
        }
        self.counter += 1;
        if self.counter % 2 == 0 {
            self.ptr += 1;
            return self.beats[self.ptr - 1].clone();
        }
        None
    }

    pub fn name(&self) -> &str {
        &self.path.as_str()[6..self.path.len()]
    }
}
