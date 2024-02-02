use std::io::{BufReader, Read, Seek};

use crate::core::token::{Offset, TokenVal};

pub struct Coliner<T: Read + Seek> {
    offset: Offset,
    last_col: usize,
    reader: BufReader<T>,
}

impl<T: Read + Seek> Coliner<T> {
    pub fn new(reader: T) -> Self {
        Self {
            offset: Offset { col: 1, row: 1 },
            last_col: 1,
            reader: BufReader::new(reader),
        }
    }

    pub fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.reader.read(&mut buf).unwrap() == 1 {
            let c = buf[0] as char;
            if c == '\n' {
                self.offset.row += 1;
                self.offset.col = 1;
            } else {
                self.offset.col += 1;
                self.last_col = self.offset.col;
            }
            c
        } else {
            '\0'
        }
    }

    pub fn put_char_back(&mut self) {
        self.reader.seek(std::io::SeekFrom::Current(-1)).unwrap();
        // A '\n' will never be put back
        self.offset.col -= 1;
        self.last_col = self.offset.col
    }

    pub fn check_next_char(&mut self, ch: char, this: TokenVal, other: TokenVal) -> TokenVal {
        if self.read_char() == ch {
            this
        } else {
            self.put_char_back();
            other
        }
    }

    pub fn current(&self) -> Offset {
        self.offset
    }
}
