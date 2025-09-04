// is even function 

fn main() {
    let ans = is_even(201);
    println!("{}", ans);
}

fn is_even(num:i32) -> bool {
    if num%2==0 {
        return true;
    } else {
        return false; 
}
}