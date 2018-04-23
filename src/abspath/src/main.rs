extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = App::new("abspath")
        .version("0.1.0")
        .author("Steven Yuhang Wang")
        .about("Convert any path to absolute path")
        .arg(Arg::with_name("path")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("relative path"))
        .get_matches();

    let raw_path = PathBuf::from(
        args.value_of("path").unwrap()
        );
    
    let abs_path = fs::canonicalize(&raw_path).unwrap();

    println!("{:?}", abs_path);
}
