/// # 测试Rust集合中的list
/// - 使用`Vec::new`方法或者`vec!`宏创建Vec
/// - 使用`push`方法添加元素
/// - 两种获取元素的方法
/// - 修改元素
/// - 遍历集合
pub fn test_vec() {
    {
        let v = vec![1, 2, 3, 4, 5];
        for i in &v {
            println!("{}", i);
        }
    }
    test_get_from_vec();
    test_new_vec();
    test_add_to_vec();
    test_enum_in_vec();
    test_modify_vec();
}
fn test_new_vec() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let v = vec![1, 2, 3, 4];
    println!("{:?}", v);
}
fn test_add_to_vec() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(4);
}
fn test_get_from_vec() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{:?}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    v.get(100);
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    v.push(6);
    println!("{:?}", first);
}
fn test_enum_in_vec() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.22),
        SpreadsheetCell::Int(431),
    ];
    println!("{:?}", row)
}
fn test_modify_vec() {
    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        *i += 50;
    }
    println!("修改集合中的元素：{:?}", v);
}

#[derive(Debug)]
/// 1. 集合中存放统一数据类型
/// 2. 可以通过**枚举类型**包装不同数据类型，实现存放不同数据类型
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
