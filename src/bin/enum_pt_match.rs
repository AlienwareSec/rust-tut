enum Shape{
    Rect(f64, f64),
    Circle(f64),
}

fn main(){ 
    let rect = Shape::Rect(2.0,4.0);
    println!("{}",calc_area(rect));
    let circ = Shape::Circle(7.0);
    println!("{}",calc_area(circ));

}

fn calc_area(shape:Shape) -> f64 {
    match shape{
        Shape::Rect(a,b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}