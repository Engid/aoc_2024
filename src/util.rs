use std::env;
use std::fs::File;
use std::io::Read;

pub fn read_input(day: u8, input_num: u8) -> String {
    let current_dir = env::current_dir().unwrap();
    // todo: better path formatting?
    let path = format!("input/day{}/input{}.txt", day, input_num);
    let file_path = current_dir.join(path);
    let mut f = File::open(file_path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    contents
}
