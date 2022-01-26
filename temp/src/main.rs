#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
fn main() {
    let r1 = Rectangle::new(4, 6);
    let r2 = r1.clone();
    let r3 = &r1;
    println!("{:?} \n{:?} \n{:?} \n", &r1, &r2, &r3);
    println!("{:?} \n{:?} \n{:?} \n", r1, r2, r3);
    println!("{:?} \n{:?} \n{:?} \n", r1, r2, r3);
}
