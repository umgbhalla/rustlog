fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let a = 6;
    let b = &a;
    let c = &b;
    let d = &c;
    println!("{} {} {} {}", a, b, c, d);
    let k = vec![4, 5, 2];
    print_type_of(&a);
    print_type_of(&b);
    print_type_of(&c);
    print_type_of(&d);
}
// fn main() {
//     let o = '+';
//     let a = 5;
//     let b = 6;
//     calc(6, '+', b);
//     calc(a, o, b);
//     calc(5, '*', 5);
//     let g: &str = "sdasda";
//     println!("{}", g);
// }
//
// fn calc(x: i32, o: char, y: i32) {
//     match o {
//         '+' => println!("{}", add(x, y)),
//         '/' => println!("{}", div(x, y)),
//         '*' => println!("{}", mul(x, y)),
//         '-' => println!("{}", sub(x, y)),
//         _ => println!("error"),
//     }
// }
// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
// fn sub(x: i32, y: i32) -> i32 {
//     x - y
// }
// fn mul(x: i32, y: i32) -> i32 {
//     x * y
// }
// fn div(x: i32, y: i32) -> i32 {
//     x / y
// }
