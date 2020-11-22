//! Music Implementation
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
/// Music Struct with name and source
#[allow(dead_code)]
pub struct Music 
{
    path: String,
    name: String,
    source: Buffered<Decoder<BufReader<File>>>,
}

impl Music
{
    pub fn new(path: &str) -> Result<Music, ()> {
        let file = File::open(path).unwrap();
        let data = BufReader::new(file);
        let source = rodio::Decoder::new(data).unwrap().buffered();
        let name = String::from(path);
        let name: Vec<_> = name.split("/").collect();
        let mut music_name = String::new();
        for iter in name {
            if  iter.to_string().contains(".mp3") {
                music_name = iter.to_string();
            }
        }
        Ok(
            Self {
                path: String::from(path),
                name: music_name,
                source,
            }
        )
    }
    
    #[allow(dead_code)]
    pub fn get_source(&self) -> Buffered<Decoder<BufReader<File>>> {
        self.source.clone()
    }
    
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

}

#[test]
fn test_music() {
    let path = "Music/Ref-rain.mp3";
    let music = Music::new(path).unwrap();
    assert_eq!(music.path, "Music/Ref-rain.mp3");
    assert_eq!(music.name(), "Ref-rain.mp3");
}