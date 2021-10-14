pub fn test_string() {
    let mut s = String::new();
    s.push_str("🤞1234567890");
    println!("字符串长度。-------{}", s.len());
    let h = &s[0..4];
    for c1 in s.chars() {
        println!("{}", c1)
    }
    eprintln!("{}", h);
    eprintln!("{}", s);
}
