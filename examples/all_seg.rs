use char_trie::Trie;
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

    let text = "我爱北京天安门，天安门上太阳升。我是中国人，我爱中国。";

    for token in trie.iter_all(text) {
        println!("{:?}", token);
    }

    let c: Vec<_> = trie.iter_all(text).map(|t| t.0).collect();
    println!("{:?}", c);
}
