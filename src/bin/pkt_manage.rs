// cargo add <package_name> will add the package to your current project
//Using chrono for this code

use chrono::{Utc, Local};

fn main(){
    let utc = Utc::now();
    let local = Local::now();
    println!("The UTC time is {}", utc);
    println!("Your local time is {}", local);
}