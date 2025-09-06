// it is used for Ok value or Err value

use std::fs::read_to_string;

fn main(){
    let ans = read_from_file_rust(String::from("rust.txt"));
    println!("{}",ans);
}

fn read_from_file_rust(file_path: String) -> String {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => data,
        Err(_err) => String::from("File not present!"),
    }
}