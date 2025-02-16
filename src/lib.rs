use std::{
    env,
    fs::File,
    io::prelude::*,
    io::{BufReader, Lines},
    path::Path,
};

/// Assuming that the input file will be the first command line arg
pub fn read_input() -> Lines<BufReader<File>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough args!");
    }

    let path = Path::new(&args[1]);
    if let Ok(file) = File::open(&path) {
        return BufReader::new(file).lines();
    } else {
        panic!("failed to open file");
    }
}
