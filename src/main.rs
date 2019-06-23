use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        panic!("wrong number of arguments. can only 1 argument for now, sorry...");
    }

    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("{}", contents);
}
