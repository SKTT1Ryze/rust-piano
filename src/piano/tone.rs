//! Tone Implementation
//! 

use std::{
    fs::File,
    io::BufReader,
};

use rodio::{
    Source,
    source::Buffered,
    Decoder,
};
use walkdir::WalkDir;

pub struct Tone {
    pub name: String,
    pub source: Buffered<Decoder<BufReader<File>>>,    
}

impl Tone {
    pub fn new(source_path: &str) -> Result<Tone, ()> {
        let file = File::open(source_path).unwrap();
        let source = BufReader::new(file);
        let source = rodio::Decoder::new(source).unwrap().buffered();
        let name = String::from(source_path);
        let name: Vec<_> = name.split("/").collect();
        let mut tone_name = String::new();
        for iter in name {
            if iter.to_string().contains(".MP3") {
                tone_name = iter.to_string();
            }
        }
        assert_eq!(tone_name.remove(tone_name.len() - 1), '3');
        assert_eq!(tone_name.remove(tone_name.len() - 1), 'P');
        assert_eq!(tone_name.remove(tone_name.len() - 1), 'M');
        assert_eq!(tone_name.remove(tone_name.len() - 1), '.');
        Ok(
            Self {
                name: tone_name,
                source,
            }
        )
    }
}
#[allow(dead_code)]
pub struct Tones<'a> {
    pitch: &'a str,
    tone_list: Vec<Tone>,
}

impl<'a> Tones<'a> {
    pub fn new(pitch: &'a str, tones_path: &'a str) -> Result<Tones<'a>, ()> {
        let mut tone_list = Vec::new();
        for entry in WalkDir::new(tones_path) {
            let entry = entry.unwrap();
            let path = String::from(entry.path().to_str().unwrap());
            if path.len() > tones_path.len() {
                let tone = Tone::new(path.as_str()).unwrap();
                tone_list.push(tone);
            }
        }
        Ok(
            Self {
                pitch,
                tone_list,
            }
        )
    }

    pub fn get_tone_source(&self, name: &'a str) -> Result<Buffered<Decoder<BufReader<File>>>, ()> {
        for tone in &self.tone_list {
            if tone.name.as_str() == name {
                return Ok(tone.source.clone());
            }
        }
        Err(())
    }
}
