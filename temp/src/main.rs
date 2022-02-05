use std::fmt::format;

#[allow(unused_variables)]
fn main() {
    let s1 = "string1";
    let s2 = String::from("lamo");
    let s3 = "dasdad".to_string();

    let slice1 = &s1[3..];
    let slice2 = &s1[..4];
    let hmm1 = &s2[..2];
    let hmm2 = &s2[3..];
    let mix = format!("{} {}", hmm1, hmm2);
    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        s1, s2, s3, slice1, slice2, mix, &hmm2
    );
    print_borrow(&s1);
}
fn print_borrow(s1: &str) {
    println!("{}", s1);
}
