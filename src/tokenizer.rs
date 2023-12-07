use crate::{Status, Trie};

/// Token is a tuple of (word, (start, end), value)
pub type Token<'a, T> = (&'a str, (usize, usize), Option<&'a T>);

pub struct MaxFrontTokenizer<'a, T> {
    text: &'a str,
    root: &'a Trie<T>,
    start: usize,
}

impl<'a, T> MaxFrontTokenizer<'a, T> {
    pub fn new(trie: &'a Trie<T>, text: &'a str) -> Self {
        Self {
            text,
            root: trie,
            start: 0,
        }
    }
}

impl<'a, T> Iterator for MaxFrontTokenizer<'a, T> {
    type Item = Token<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut temp_end = 0;
        let mut temp_value = None;
        let mut temp_trie = self.root;
        let mut first_len = 0;

        loop {
            if self.start >= self.text.len() {
                return None;
            }

            let cs = self.text[self.start..].char_indices();

            for (i, c) in cs {
                let c_len = c.len_utf8();
                if first_len == 0 {
                    first_len = c_len;
                }

                match temp_trie.char_get(c) {
                    Some(trie) => match trie.status {
                        Status::End => {
                            temp_end = self.start + i + c_len;
                            temp_value = trie.value.as_ref();
                            temp_trie = trie;
                            continue;
                        }
                        Status::LastEnd => {
                            println!("lastend");
                            let start = self.start;
                            let end: usize = self.start + i + c_len;
                            self.start = end;
                            return Some((
                                &self.text[start..end],
                                (start, end),
                                trie.value.as_ref(),
                            ));
                        }
                        Status::Not => {
                            temp_trie = trie;
                            continue;
                        }
                    },
                    None => {
                        if temp_end > 0 {
                            let start = self.start;
                            self.start = temp_end;
                            return Some((
                                &self.text[start..temp_end],
                                (start, temp_end),
                                temp_value,
                            ));
                        } else {
                            self.start += first_len;
                            temp_trie = self.root;
                            first_len = 0;
                            break;
                        }
                    }
                }
            }
            if temp_end > 0 {
                let start = self.start;
                self.start = temp_end;
                return Some((&self.text[start..temp_end], (start, temp_end), temp_value));
            }
        }
    }
}

pub struct AllTokenizer<'a, T> {
    text: &'a str,
    root: &'a Trie<T>,
    trie: &'a Trie<T>,
    start: usize,
    end: usize,
    first_len: usize,
}

impl<'a, T> AllTokenizer<'a, T> {
    pub fn new(trie: &'a Trie<T>, text: &'a str) -> Self {
        Self {
            text,
            root: trie,
            trie,
            start: 0,
            end: 0,
            first_len: 0,
        }
    }
}

impl<'a, T> Iterator for AllTokenizer<'a, T> {
    type Item = Token<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut first = true;
        loop {
            if self.start >= self.text.len() {
                return None;
            }

            let cs = self.text[self.end..].char_indices();

            for (i, c) in cs {
                let c_len = c.len_utf8();
                if first {
                    self.first_len = c_len;
                    first = false;
                }

                match self.trie.char_get(c) {
                    Some(trie) => match trie.status {
                        Status::End => {
                            self.end = self.end + i + c_len;
                            self.trie = trie;
                            return Some((
                                &self.text[self.start..self.end],
                                (self.start, self.end),
                                trie.value.as_ref(),
                            ));
                        }
                        Status::LastEnd => {
                            let start = self.start;
                            let end = self.end + i + c_len;
                            self.start = self.start + self.first_len;
                            self.end = self.start;
                            self.trie = self.root;
                            return Some((
                                &self.text[start..end],
                                (start, end),
                                trie.value.as_ref(),
                            ));
                        }
                        Status::Not => {
                            self.trie = trie;
                            continue;
                        }
                    },
                    None => {
                        break;
                    }
                }
            }
            self.start += self.first_len;
            self.end = self.start;
            self.trie = self.root;
        }
    }
}

pub struct Tokenizer<'a, T> {
    text: &'a str,
    root: &'a Trie<T>,
    trie: &'a Trie<T>,
    start: usize,
    end: usize,
    first_len: usize,
}

impl<'a, T> Tokenizer<'a, T> {
    pub fn new(trie: &'a Trie<T>, text: &'a str) -> Self {
        Self {
            text,
            root: trie,
            trie,
            start: 0,
            end: 0,
            first_len: 0,
        }
    }

    pub fn front_max(&mut self) -> Option<Token<'a, T>> {
        let mut first = true;
        let mut temp_end = 0;
        let mut temp_value = None;
        let mut temp_trie = self.root;
        loop {
            if self.start >= self.text.len() {
                return None;
            }

            let cs = self.text[self.start..].char_indices();

            for (i, c) in cs {
                let c_len = c.len_utf8();
                if first {
                    self.first_len = c_len;
                    first = false;
                }

                match temp_trie.char_get(c) {
                    Some(trie) => match trie.status {
                        Status::End => {
                            temp_end = self.start + i + c_len;
                            temp_value = trie.value.as_ref();
                            temp_trie = trie;
                            continue;
                        }
                        Status::LastEnd => {
                            println!("lastend");
                            let start = self.start;
                            let end: usize = self.start + i + c_len;
                            self.start = end;
                            return Some((
                                &self.text[start..end],
                                (start, end),
                                trie.value.as_ref(),
                            ));
                        }
                        Status::Not => {
                            temp_trie = trie;
                            continue;
                        }
                    },
                    None => {
                        if temp_end > 0 {
                            let start = self.start;
                            self.start = temp_end;
                            return Some((
                                &self.text[start..temp_end],
                                (start, temp_end),
                                temp_value,
                            ));
                        } else {
                            self.start += self.first_len;
                            self.trie = self.root;
                            first = true;
                            break;
                        }
                    }
                }
            }
            if temp_end > 0 {
                let start = self.start;
                self.start = temp_end;
                return Some((&self.text[start..temp_end], (start, temp_end), temp_value));
            }
        }
    }

    pub fn all(&mut self) -> Option<Token<'a, T>> {
        let mut first = true;
        loop {
            if self.start >= self.text.len() {
                return None;
            }

            let cs = self.text[self.end..].char_indices();

            for (i, c) in cs {
                let c_len = c.len_utf8();
                if first {
                    self.first_len = c_len;
                    first = false;
                }

                match self.trie.char_get(c) {
                    Some(trie) => match trie.status {
                        Status::End => {
                            self.end = self.end + i + c_len;
                            self.trie = trie;
                            return Some((
                                &self.text[self.start..self.end],
                                (self.start, self.end),
                                trie.value.as_ref(),
                            ));
                        }
                        Status::LastEnd => {
                            let start = self.start;
                            let end = self.end + i + c_len;
                            self.start = self.start + self.first_len;
                            self.end = self.start;
                            self.trie = self.root;
                            return Some((
                                &self.text[start..end],
                                (start, end),
                                trie.value.as_ref(),
                            ));
                        }
                        Status::Not => {
                            self.trie = trie;
                            continue;
                        }
                    },
                    None => {
                        break;
                    }
                }
            }
            self.start += self.first_len;
            self.end = self.start;
            self.trie = self.root;
        }
    }
}
