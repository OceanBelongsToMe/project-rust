use collections_code::{handler_error, hashmap, list, string};
use std::fs::File;
fn main() {
    list::test_vec();
    string::test_string();
    hashmap::new_hashmap();
    hashmap::get_from_map();
    let path = "./target/hello.txt";
    handler_error::open_file(path);
    let f = File::open(&path);
    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("{:?}", &err);
        }
    };
    println!("{:?},{}", f, path);
    let s = handler_error::read_file(path).unwrap();
    println!("{}", s)
}
