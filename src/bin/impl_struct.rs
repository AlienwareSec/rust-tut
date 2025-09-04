struct Rect{
    len: u32,
    breadth: u32,
}

impl Rect{
    fn area(&self) -> u32 {
        self.len * self.breadth
    }
    fn peri(&self) -> u32 {
        2 * (self.len + self.breadth)
    }
}


fn main(){
    let phone = Rect{
        len: 20,
        breadth: 40,

    };
    println!("the area of the phone is {} & perimeter is {}", phone.area(), phone.peri());
}