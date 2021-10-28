use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

pub fn open_file(path: &str) {
    let f = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("{:?}", &error);
            })
        } else {
            panic!("{:?}", error);
        }
    });
    println!("{:?}", f);
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
