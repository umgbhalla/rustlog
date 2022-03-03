pub fn print(limit: u8) {
    let numbers = generate_seq(limit);
    output_seq(&numbers)
}
fn generate_seq(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
}
fn output_seq(numbers: &[u8]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}

#[test]
fn should_generate_seq() {
    let result = generate_seq(3);
    assert_eq!(result, &[1, 2, 3]);
}
