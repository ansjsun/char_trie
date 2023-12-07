//! # 一个trie树结构，并且实现了，正向最大匹配和全词匹配
//!
//!
//! Licensed under either of
//! * Apache License, Version 2.0,
//! (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
//! at your option.
//!
//! ## Examples
//!
//! All examples are in the [sub-repository](https://github.com/ansjsun/char_trie/examples), located in the examples directory.
//!
//!
//! ```example
//! cd test
//! cargo run --package char_trie --example example
//! ```
//!

pub mod tokenizer;

use std::str::Chars;

use tokenizer::AllTokenizer;
use tokenizer::MaxFrontTokenizer;

pub type Tokenizer<'a, T> = tokenizer::Tokenizer<'a, T>;

/// 叶子结点状态，
/// Not 不是一个词
/// End 是一个词，但不是最后一个词
/// LastEnd 是一个词，也是最后一个词
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum Status {
    #[default]
    Not,
    End,
    LastEnd,
}

/// 叶子结点，包含一个字符，状态，值，和子节点
/// Big 是一个大的节点，用于存储所有的字符，可以有效加速查询和词典加载速度，但是较耗费空间
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

/// Trie 树，用于存储词典
/// # Examples
/// ```rust
/// use char_trie::Trie;
/// let mut trie = Trie::default();
/// trie.insert("中国人", "cns");
/// assert_eq!(trie.get("中国人"), Some("cns").as_ref());
/// assert_eq!(trie.get("中国"), None);
///
#[derive(Debug, Default)]
pub struct Trie<T> {
    c: char,
    status: Status,
    value: Option<T>,
    leafs: Leafs<T>,
}

impl<T> Trie<T> {
    /// 创建一个新的 Trie 树, 对于超大词典作了优化，可以有效加速词典加载速度，但是较耗费空间
    /// # Examples
    /// ```rust
    /// use char_trie::Trie;
    /// let mut trie = Trie::new_big();
    /// trie.insert("中国人", "cns");
    /// assert_eq!(trie.get("中国人"), Some("cns").as_ref());
    /// assert_eq!(trie.get("中国"), None);
    /// ```
    pub fn new_big() -> Self {
        Self {
            c: '\0',
            status: Status::Not,
            value: None,
            leafs: Leafs::Big((0..65536).map(|_| None).collect()),
        }
    }

    /// 插入一个词到trie树中
    /// key 词
    /// value 词的值
    /// # Examples
    /// ```rust
    /// use char_trie::Trie;
    /// let mut trie = Trie::default();
    /// trie.insert("中国人", "cns");
    /// assert_eq!(trie.get("中国人"), Some("cns").as_ref());
    /// assert_eq!(trie.get("中国"), None);
    /// ```
    pub fn insert(&mut self, key: &str, value: T) {
        if key.is_empty() {
            return;
        }
        self.inner_insert(key.chars(), value)
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

    fn new(c: char) -> Self {
        Trie {
            c,
            status: Status::Not,
            value: None,
            leafs: Leafs::Small(Vec::new()),
        }
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

    /// 实现了全词匹配， 如词典中包含 【中国，国人，中国人】 三个词，那么对于文本 “我是中国人” 将返回
    /// [中国，国人，中国人]
    /// # Examples
    /// ```rust
    /// use char_trie::Trie;
    /// let mut trie = Trie::default();
    /// trie.insert("中国人", "cns");
    /// trie.insert("中国", "cn");
    /// trie.insert("国人", "gr");
    /// let text = "我是中国人";
    /// let tokens: Vec<_> = trie.iter_all(text).map(|t| t.0).collect();
    /// assert_eq!(tokens, vec!["中国", "中国人", "国人"]);
    /// ```
    pub fn iter_all<'a>(&'a self, text: &'a str) -> AllTokenizer<'a, T> {
        AllTokenizer::new(self, text)
    }

    /// 实现了正向最大匹配， 如词典中包含 【中国，国人，中国人】 三个词，那么对于文本 “我是中国人” 将返回
    /// [中国人]
    /// # Examples
    /// ```rust
    /// use char_trie::Trie;
    /// let mut trie = Trie::default();
    /// trie.insert("中国人", "cns");
    /// trie.insert("中国", "cn");
    /// trie.insert("国人", "gr");
    /// let text = "我是中国人";
    /// let tokens: Vec<_> = trie.iter_max(text).map(|t| t.0).collect();
    /// assert_eq!(tokens, vec!["中国人"]);
    /// ```
    pub fn iter_max<'a>(&'a self, text: &'a str) -> MaxFrontTokenizer<'a, T> {
        MaxFrontTokenizer::new(self, text)
    }
}
