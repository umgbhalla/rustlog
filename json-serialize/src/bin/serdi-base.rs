extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
    points: Vec<Point>,
    valid: bool,
    length: f32,
    desc: String,
}
fn main() {
    let x: i32 = 5;
    let xs = serde_json::to_string(&x).unwrap();
    println!("i32 number {} serializes into string {}", x, xs);
    let xd: i32 = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x: f32 = 3.14;
    let xs = serde_json::to_string(&x).unwrap();
    println!("f32 number {} serializes into string {}", x, xs);

    let x: Vec<u8> = vec![1, 2, 3];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<u8> {:?} serializes into string {}", x, xs);
    let xd: Vec<u8> = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x: Vec<f32> = vec![3.141, 2.718, 1.618];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<f32> {:?} serializes into string {}", x, xs);
    let x: (i32, &str, f32, bool) = (1, "hello", 4.5, true);
    let xs = serde_json::to_string(&x).unwrap();
    println!("tuple {:?} serializes into string {}", x, xs);
    let xd: (i32, &str, f32, bool) = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x = ((1u8, 2u16), (3.141f32, 'a'), true);
    let xs = serde_json::to_string(&x).unwrap();
    println!("nested tuple {:?} serializes into string {}", x, xs);

    let point1: Point = Point { x: 1.0, y: 2.0 };
    let point2: Point = Point { x: 3.0, y: 4.0 };
    let point1s = serde_json::to_string(&point1).unwrap();
    let point2s = serde_json::to_string(&point2).unwrap();
    println!("struct Point serializes into string {}", point1s);
    println!("struct Point serializes into string {}", point2s);

    let length = ((point1.x - point2.x) * (point1.x - point2.x)
        + (point1.y - point2.y) * (point1.y - point2.y))
        .sqrt();
    let valid = !(length == 0.0);
    let line = Line {
        points: vec![point1, point2],
        valid,
        length,
        desc: "a thin line".to_string(),
    };
    let lines = serde_json::to_string(&line).unwrap();
    println!("struct Line serializes into string {}", lines);

    let lined: Line = serde_json::from_str(&lines).unwrap();
    assert_eq!(lined.desc, "a thin line");
    assert_eq!(lined.points[1].x, 3.0);

    let x: i32 = 21474836;
    let xs: Vec<u8> = bincode::serialize(&x).unwrap();
    println!("i32 number {} serializes into byte array {:?}", x, xs);
    let xd: i32 = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x: f32 = 3.14;
    let xs = bincode::serialize(&x).unwrap();
    println!("f32 number {} serializes into byte array {:?}", x, xs);

    let x: Vec<u8> = vec![1, 2, 3];
    let xs = bincode::serialize(&x).unwrap();
    println!("Vec<u8> {:?} serializes into byte array {:?}", x, xs);
    let xd: Vec<u8> = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x: Vec<f32> = vec![3.141, 2.718, 1.618];
    let xs = bincode::serialize(&x).unwrap();
    println!("Vec<f32> {:?} serializes into byte array {:?}", x, xs);
    let xd: Vec<f32> = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x: (i32, &str, f32, bool) = (1, "hello", 4.5, true);
    let xs = bincode::serialize(&x).unwrap();
    println!("tuple {:?} serializes into byte array {:?}", x, xs);
    let xd: (i32, &str, f32, bool) = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x = ((1u8, 2u16), (3.141f32, 'a'), true);
    let xs = bincode::serialize(&x).unwrap();
    println!("nested tuple {:?} serializes into byte array {:?}", x, xs);

    let point1: Point = Point { x: 1.0, y: 2.0 };
    let point2: Point = Point { x: 3.0, y: 4.0 };
    let point1s = bincode::serialize(&point1).unwrap();
    let point2s = bincode::serialize(&point2).unwrap();
    println!("struct Point serializes into byte array {:?}", point1s);
    println!("struct Point serializes into byte array {:?}", point2s);

    let length = ((point1.x - point2.x) * (point1.x - point2.x)
        + (point1.y - point2.y) * (point1.y - point2.y))
        .sqrt();
    let valid = !(length == 0.0);
    let line = Line {
        points: vec![point1, point2],
        valid,
        length,
        desc: "a thin line".to_string(),
    };
    let lines = bincode::serialize(&line).unwrap();
    println!("struct Line serializes into byte array {:?}", lines);

    let lined: Line = bincode::deserialize(&lines).unwrap();
    assert_eq!(lined.desc, "a thin line");
    assert_eq!(lined.points[1].x, 3.0);
}
