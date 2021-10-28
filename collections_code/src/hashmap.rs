use std::collections::HashMap;

/// Rust中创建HashMap
pub fn new_hashmap() {
    let mut new_map = HashMap::new();
    new_map.insert("k", 1);
    new_map.insert("k1", 2);
    let s = String::from("kk");
    new_map.insert(&s, 4);
    println!("{:?}", new_map);
}
/// 通过解引符号，能修改变量值
pub fn get_from_map() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // sleep(Duration::from_secs(3));
        *count += 1;
    }
    println!("{:?}", map.get_mut("world"));
    println!("{:?}", map);
}
