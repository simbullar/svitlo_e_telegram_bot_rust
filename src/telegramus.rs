use std::io::{stdout, Write};

use curl::easy::Easy;


pub fn send(val: &mut String){
    // Write the contents of rust-lang.org to stdout
    let mut easy = Easy::new();
    easy.url(val).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
   easy.perform().unwrap();
}