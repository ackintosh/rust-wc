use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("wrong number of arguments. can only 1 argument for now, sorry...");
    }

    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: usize = contents.lines().count();
    let words: usize = contents.split_whitespace().count();
    let bytes: usize = contents.len();

    println!(
        "{0: >7} {1: >7} {2: >7} {3: >7}",
        lines,
        words,
        bytes,
        filename
    );
}
