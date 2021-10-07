pub fn test_string() {
    let mut s = String::new();
    s.push_str("1234567890🤞");
    println!("字符串长度。-------{}", s.len());
    let h = &s[10..14];
    for c in s.chars() {
        println!("{}", c)
    }
    println!("{}", h);
    println!("{}", s);
}
