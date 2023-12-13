use char_trie::Trie;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let mut trie = Trie::default();
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

    println!("load dict use {:?}", start.elapsed());

    let file = std::fs::read_to_string("dict/big_text.txt").unwrap();

    let start = std::time::Instant::now();
    let mut len = 0;

    file.lines().for_each(|line| {
        let result: Vec<_> = trie.iter_max(line).map(|t| t.0).collect();
        len += result.len();
        println!("{:?}", result);
    });

    println!("text parse token:{} use {:?}", len, start.elapsed());
}
