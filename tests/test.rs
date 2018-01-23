extern crate lumberjack as lj;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use lj::read_batch;

#[test]
fn test_extract_success() {
    let mut data = Vec::new();
    File::open(&Path::new(&"tests/data"))
        .and_then(|mut file| file.read_to_end(&mut data))
        .unwrap();
    read_batch(data.as_slice()).unwrap();
}
