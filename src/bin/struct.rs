struct User{
    name: String,
    username: String,
    email: String,
    user_id: i32,
    is_indian: bool,
}

fn main(){
    let pawan = User{
        name: String::from("Pawan"),
        username: String::from("alienwaresec"),
        email: String::from("pawan@gmail.com"),
        user_id: 21,
        is_indian: false,
    };

    println!("Is {} Indian? {}", pawan.name, pawan.is_indian);
    println!("{}", pawan.is_indian);
}