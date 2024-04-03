pub trait Danmaku {
    fn new(x: f64, y: f64, speed: f64) -> Self;
    fn update(&mut self);
    
    
    fn distance(&self, other: &Self) -> f64;


}

fn main() {
    println!("Hello, world!");
}
