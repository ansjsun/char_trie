use crate::{Status, Trie};

pub struct Tokenizer<'a, T> {
    text: &'a str,
    root: &'a Trie<T>,
    trie: &'a Trie<T>,
    start: usize,
    end: usize,
    first_len: usize,
}

pub type Token<'a, T> = (&'a str, (usize, usize), Option<&'a T>);

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
        // let mut cs = self.text[self.index..].char_indices();

        // let mut trie = self.trie.inner_get(cs)?;
        // let mut index = self.index;

        // while trie.status != Status::End && trie.status != Status::LastEnd {
        //     index += 1;
        //     trie = trie.inner_get(self.text[index..].chars())?;
        // }

        // self.index = index + 1;
        // trie.value.as_ref()
        todo!()
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
