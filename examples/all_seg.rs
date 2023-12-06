use char_trie::{Tokenizer, Trie};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    // let mut trie = Trie::default();
    let mut trie = Trie::new_big();

    let start = std::time::Instant::now();

    for line in BufReader::new(File::open("dict/default.dic").unwrap()).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 3 {
            trie.insert(
                parts[0],
                (parts[1].to_string(), parts[2].parse::<i32>().unwrap()),
            );
        }
    }

    trie.insert("中国人", (String::from("ud"), 10000));

    println!("load dict use {:?}", start.elapsed());

    // let text = "我是中国人，我爱中国。";

    let text = "我爱北京天安门，天安门上太阳升。";
    let mut tokizer = Tokenizer::new(&trie, text);

    while let Some(token) = tokizer.all() {
        println!("{:?}", token);
    }

    // let mut trie = Trie::default();

    // trie.insert("abc", 1);
    // trie.insert("abcd", 2);
    // trie.insert("abcde", 3);

    // println!("{:?}", trie.get("abcd"));

    // let mut trie: TrieTree<i32> = TrieTree::default();

    // trie.insert("abc", 1);
    // trie.insert("abcd", 2);
    // trie.insert("abcde", 3);

    // println!("{:?}", trie.get("abcd"));
}
