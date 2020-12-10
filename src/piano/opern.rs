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

pub struct Opern<'a> {
    _path: &'a str,
    beats: Vec<Option<Buffered<Decoder<BufReader<File>>>>>,
    ptr: usize,
    counter: usize,
}

impl<'a> Opern<'a> {
    pub fn new(path: &'a str, tones: &Tones) -> Opern<'a> {
        let context = fs::read_to_string(path).unwrap();
        let words: Vec<_> = context.split(' ').collect();
        let mut beats = Vec::new();
        for word in words {
            let word = word.to_string();
            match word.as_str() {
                "-" => {
                    beats.push(None);
                },
                _ => {
                    let beat = tones.get_tone_source(word.as_str()).unwrap();
                    beats.push(Some(beat));
                }
            }
        }
        Self {
            _path: path,
            beats,
            ptr: 0,
            counter: 0,
        }
    }

    pub fn count(&mut self) -> Option<Buffered<Decoder<BufReader<File>>>> {
        if self.ptr == self.beats.len() {
            return None;
        }
        self.counter += 1;
        if self.counter == 2 {
            self.ptr += 1;
            self.counter = 0;
            return self.beats[self.ptr - 1].clone();
        }
        None
    }
}
