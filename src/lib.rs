mod tokenizer;

use std::str::Chars;

pub type Tokenizer<'a, T> = tokenizer::Tokenizer<'a, T>;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum Status {
    #[default]
    Not,
    End,
    LastEnd,
}

#[derive(Debug)]
enum Leafs<T> {
    Big(Vec<Option<Trie<T>>>),
    Small(Vec<Trie<T>>),
}

impl<T> Default for Leafs<T> {
    fn default() -> Self {
        Leafs::Small(Vec::new())
    }
}

impl<T> Leafs<T> {
    fn binary_search(&self, c: char) -> Result<usize, usize> {
        match self {
            Leafs::Big(v) => {
                let index = c as usize;
                if v[index].is_none() {
                    return Err(index);
                } else {
                    return Ok(index);
                }
            }
            Leafs::Small(v) => v.binary_search_by(|t| c.cmp(&t.c)),
        }
    }

    fn get_mut(&mut self, index: usize) -> &mut Trie<T> {
        match self {
            Leafs::Big(v) => v[index].as_mut().unwrap(),
            Leafs::Small(v) => v.get_mut(index).unwrap(),
        }
    }

    pub fn get_uncheck(&self, index: usize) -> &Trie<T> {
        match self {
            Leafs::Big(v) => v[index].as_ref().unwrap(),
            Leafs::Small(v) => v.get(index).unwrap(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Trie<T>> {
        match self {
            Leafs::Big(v) => v.get(index)?.as_ref(),
            Leafs::Small(v) => v.get(index),
        }
    }

    fn insert(&mut self, index: usize, c: Trie<T>) {
        match self {
            Leafs::Big(v) => {
                v[index] = Some(c);
            }
            Leafs::Small(v) => {
                v.insert(index, c);
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Leafs::Big(_) => false,
            Leafs::Small(v) => v.is_empty(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Trie<T> {
    c: char,
    status: Status,
    value: Option<T>,
    leafs: Leafs<T>,
}

impl<T> Trie<T> {
    pub fn new_big() -> Self {
        Self {
            c: '\0',
            status: Status::Not,
            value: None,
            leafs: Leafs::Big((0..65536).map(|_| None).collect()),
        }
    }

    fn new(c: char) -> Self {
        Trie {
            c,
            status: Status::Not,
            value: None,
            leafs: Leafs::Small(Vec::new()),
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        if key.is_empty() {
            return;
        }
        self.inner_insert(key.chars(), value)
    }

    fn inner_insert(&mut self, cs: Chars<'_>, value: T) {
        let mut trie = self;

        let mut has_end = false;

        for c in cs {
            if has_end {
                trie.status = Status::End;
                has_end = false;
            }
            match trie.leafs.binary_search(c) {
                Ok(index) => {
                    trie = trie.leafs.get_mut(index);
                    if trie.status == Status::LastEnd {
                        has_end = true;
                    }
                }
                Err(index) => {
                    trie.leafs.insert(index, Trie::new(c));
                    trie = trie.leafs.get_mut(index);
                }
            }
        }

        trie.value = Some(value);
        if trie.leafs.is_empty() {
            trie.status = Status::LastEnd;
        } else {
            trie.status = Status::End;
        }
        return;
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        if key.is_empty() {
            return None;
        }

        let trie = self.inner_get(key.chars())?;
        if trie.status == Status::End || trie.status == Status::LastEnd {
            return trie.value.as_ref();
        }
        return None;
    }

    fn inner_get(&self, cs: Chars<'_>) -> Option<&Self> {
        let mut trie = self;

        for c in cs {
            match trie.leafs.binary_search(c) {
                Ok(index) => {
                    trie = trie.leafs.get_uncheck(index);
                }
                Err(_) => {
                    return None;
                }
            }
        }

        Some(trie)
    }

    pub fn char_get(&self, c: char) -> Option<&Self> {
        match self.leafs.binary_search(c) {
            Ok(index) => self.leafs.get(index),
            Err(_) => None,
        }
    }
}
