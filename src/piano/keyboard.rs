//! KeyBoard Implementation
//! 
use termion::event::Key;

pub struct PKey<'a> {
    pub key: Key,
    pub tone_name: Option<&'a str>,
    pub pressed: bool,
}

impl<'a> PKey<'a> {
    pub fn new(key: &Key) -> PKey<'a> {
        let tone_name = match key {
            Key::Backspace => None,
            Key::Left => Some("-1"),
            Key::Right => Some("-3"),
            Key::Up => Some("-4"),
            Key::Down => Some("-2"),
            Key::Home => Some("++5"),
            Key::End => Some("++2"),
            Key::PageUp => Some("++6"),
            Key::PageDown => Some("++3"),
            Key::BackTab => None,
            Key::Delete => Some("++1"),
            Key::Insert => Some("++4"),
            Key::F(_) => None,
            Key::Char(c) => {
                match c {
                    '1' => Some("1"),
                    '2' => Some("2"),
                    '3' => Some("3"),
                    '4' => Some("4"),
                    '5' => Some("5"),
                    '6' => Some("6"),
                    '7' => Some("7"),
                    '8' => Some("+1"),
                    '9' => Some("+2"),
                    '0' => Some("+3"),
                    '-' => Some("+4"),
                    '=' => Some("+5"),
                    'a' => Some("--1"),
                    'b' => Some("---5"),
                    'c' => Some("---3"),
                    'd' => Some("--3"),
                    'e' => Some("-3"),
                    'f' => Some("--4"),
                    'g' => Some("--5"),
                    'h' => Some("--6"),
                    'i' => Some("1"),
                    'j' => Some("--7"),
                    'k' => Some("-1"),
                    'l' => Some("-2"),
                    'm' => Some("---7"),
                    'n' => Some("---6"),
                    'o' => Some("2"),
                    'p' => Some("3"),
                    'q' => Some("-1"),
                    'r' => Some("-4"),
                    's' => Some("--2"),
                    't' => Some("-5"),
                    'u' => Some("-7"),
                    'v' => Some("---4"),
                    'w' => Some("-2"),
                    'x' => Some("---2"),
                    'y' => Some("-6"),
                    'z' => Some("---1"),
                    '[' => Some("4"),
                    ']' => Some("5"),
                    '\\' => Some("6"),
                    ';' => Some("-3"),
                    '\'' => Some("-4"),
                    ',' => Some("--1"),
                    '.' => Some("--2"),
                    '/' => Some("--3"),
                    '+' => Some("+3"),
                    '\n' => Some("-7"),
                    _ => None
                }
            },
            Key::Alt(a) => None,
            Key::Ctrl(c) => None,
            Key::Null => None,
            Key::Esc => None,
            Key::__IsNotComplete => None
        };
        Self {
            key: key.clone(),
            tone_name,
            pressed: false,
        }
    }
}

pub struct KeyBoard<'a> {
    keys: Vec<Vec<PKey<'a>>>,
}

impl<'a> KeyBoard<'a> {
    pub fn new() -> KeyBoard<'a> {
        let mut keys = Vec::new();
        let key_line = vec![
            Key::Esc,
            Key::F(1),
            Key::F(2),
            Key::F(3),
            Key::F(4),
            Key::F(5),
            Key::F(6),
            Key::F(7),
            Key::F(8),
            Key::F(9),
            Key::F(10),
            Key::F(11),
            Key::F(12),
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
        ];
        let mut pkey_line_0 = Vec::new();
        for key in key_line {
            let pkey = PKey::new(&key);
            pkey_line_0.push(pkey);
        }

        let key_line = vec![
            Key::Char('~'),
            Key::Char('1'),
            Key::Char('2'),
            Key::Char('3'),
            Key::Char('4'),
            Key::Char('5'),
            Key::Char('6'),
            Key::Char('7'),
            Key::Char('8'),
            Key::Char('9'),
            Key::Char('0'),
            Key::Char('-'),
            Key::Char('='),
            Key::Backspace,
            Key::Insert,
            Key::Home,
            Key::PageUp,
            Key::Null,
            Key::Char('/'),
            Key::Char('*'),
            Key::Char('-'),
        ];
        let mut pkey_line_1 = Vec::new();
        for key in key_line {
            let pkey = PKey::new(&key);
            pkey_line_1.push(pkey);
        }

        let key_line = vec![
            Key::__IsNotComplete,
            Key::Char('q'),
            Key::Char('w'),
            Key::Char('e'),
            Key::Char('r'),
            Key::Char('t'),
            Key::Char('y'),
            Key::Char('u'),
            Key::Char('i'),
            Key::Char('o'),
            Key::Char('p'),
            Key::Char('['),
            Key::Char(']'),
            Key::Char('\\'),
            Key::Delete,
            Key::End,
            Key::PageDown,
            Key::Char('7'),
            Key::Char('8'),
            Key::Char('9'),
            Key::Char('+'),
        ];
        let mut pkey_line_2 = Vec::new();
        for key in key_line {
            let pkey = PKey::new(&key);
            pkey_line_2.push(pkey);
        }

        let key_line = vec![
            Key::__IsNotComplete,
            Key::Char('a'),
            Key::Char('s'),
            Key::Char('d'),
            Key::Char('f'),
            Key::Char('g'),
            Key::Char('h'),
            Key::Char('j'),
            Key::Char('k'),
            Key::Char('l'),
            Key::Char(';'),
            Key::Char('\''),
            Key::Char('\n'),
            Key::Char('4'),
            Key::Char('5'),
            Key::Char('6'),
            Key::Char('+'),
        ];
        let mut pkey_line_3 = Vec::new();
        for key in key_line {
            let pkey = PKey::new(&key);
            pkey_line_3.push(pkey);
        }

        let key_line = vec![
            Key::__IsNotComplete,
            Key::Char('z'),
            Key::Char('x'),
            Key::Char('c'),
            Key::Char('v'),
            Key::Char('b'),
            Key::Char('n'),
            Key::Char('m'),
            Key::Char(','),
            Key::Char('.'),
            Key::Char('/'),
            Key::__IsNotComplete,
            Key::Up,
            Key::Char('1'),
            Key::Char('2'),
            Key::Char('3'),
            Key::Char('\n'),
        ];
        let mut pkey_line_4 = Vec::new();
        for key in key_line {
            let pkey = PKey::new(&key);
            pkey_line_4.push(pkey);
        }

        let key_line = vec![
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::__IsNotComplete,
            Key::Left,
            Key::Down,
            Key::Right,
            Key::Char('0'),
            Key::Char('.'),
            Key::Char('\n'),
        ];
        let mut pkey_line_5 = Vec::new();
        for key in key_line {
            let pkey = PKey::new(&key);
            pkey_line_5.push(pkey);
        }
        keys.push(pkey_line_0);
        keys.push(pkey_line_1);
        keys.push(pkey_line_2);
        keys.push(pkey_line_3);
        keys.push(pkey_line_4);
        keys.push(pkey_line_5);
        Self {
            keys
        }
    }

    pub fn get_key_pos(&self, key: Key) -> Vec<(usize, usize)> {
        let mut pos = Vec::new();
        for i in 0..self.keys.len() {
            for j in 0..self.keys[i].len() {
                if key.eq(&self.keys[i][j].key) {
                    pos.push((i, j));
                }
            }
        }
        pos
    }

    pub fn press(&mut self, key: Key) -> Option<&'a str>{
        let pos_vec = self.get_key_pos(key);
        if pos_vec.is_empty() {
            return None;
        }
        for pos in &pos_vec {
            self.keys[pos.0][pos.1].pressed = true;
        }
        self.keys[pos_vec[0].0][pos_vec[0].1].tone_name
    }

    pub fn is_pressed(&self, key_pos: (usize, usize)) -> bool {
        if key_pos.0 < self.keys.len() {
            if key_pos.1 < self.keys[key_pos.0].len() {
                return self.keys[key_pos.0][key_pos.1].pressed;
            } else {
                return false;
            }
        }
        false
    }

    pub fn clear_pressed(&mut self) {
        for key_line in &mut self.keys {
            for pkey in key_line {
                pkey.pressed = false;
            }
        }
    }

}
