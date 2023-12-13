# 一个trie树结构，并且实现了，正向最大匹配和全词匹配

````
use char_trie::Trie;

pub fn main() {
    let mut trie = Trie::default();

    trie.insert("中国人", String::from("ud"));
    trie.insert("中国", String::from("ud"));
    trie.insert("我", String::from("ud"));
    trie.insert("是", String::from("ud"));
    trie.insert("爱", String::from("ud"));
    trie.insert("北京", String::from("ud"));
    trie.insert("天安门", String::from("ud"));
    trie.insert("天安", String::from("ud"));
    trie.insert("安门", String::from("ud"));
    trie.insert("上", String::from("ud"));
    trie.insert("太阳", String::from("ud"));
    trie.insert("升", String::from("ud"));

    let text = "我爱北京天安门，天安门上太阳升。我是中国人，我爱中国。";

    let c: Vec<_> = trie.iter_all(text).map(|t| t.0).collect();
    //["我", "爱", "北京", "天安", "天安门", "安门", "天安", "天安门", "安门", "上", "太阳", "升", "我", "是", "中国", "中国人", "我", "爱", "中国"]
    println!("{:?}", c);

    let c: Vec<_> = trie.iter_max(text).map(|t| t.0).collect();

    //["我", "爱", "北京", "天安门", "天安门", "上", "太阳", "升", "我", "是", "中国人", "我", "爱", "中国"]
    println!("{:?}", c);
}

```


性能：加载30万辞典。2121857字耗时 89.604137ms 
