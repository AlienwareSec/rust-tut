fn main(){
    let name = String::from("Pawan");
    let len = get_str_len(name);
    println!("The length of your name is {}", len);
}


fn get_str_len(str: String) -> usize{
    str.chars().count()
}