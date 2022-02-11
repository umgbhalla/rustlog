mod fraction;
use fraction::Fraction;
use std::io::{self, Write};

fn main() {
    println!("Enter number of variables n");

    // inpute n from user and store
    let n = inputn();
    println!("Enter one by one {} equations", n);

    // mut string buffer for stdin
    let mut buf = String::new();
    // mut matrix [2d Vector of Fraction]
    let mut _matrix: Vec<Vec<Fraction>> = Vec::new();

    for _i in 0..n {
        // mut temporary Fraction Vector
        let mut _vec_buf: Vec<Fraction> = Vec::new();
        // loop to scan "x0 x1 x2 x3 ... xn K" as str in buf
        loop {
            // scan "x0 x1 x2 x3 ... xn K" as str in buf
            io::stdin()
                .read_line(&mut buf)
                .ok()
                .expect("read_line error");
            if buf.split(" ").collect::<Vec<&str>>().len() > n {
                break;
            } else {
                println!("Not a valid equation");
                println!("Expected sytax x0 x1 x2 x3 ... xn K");
            }
        }
        for j in buf.split(" ").collect::<Vec<&str>>() {
            let _bf64: f64 = j.trim().parse().unwrap();
            let _ffraction = Fraction::new(_bf64, 1_f64);
            _vec_buf.push(_ffraction);
        }
        _matrix.push(_vec_buf);
        buf = String::new();
    }
}

fn inputn() -> usize {
    let mut n: usize;
    let mut buf = String::new();
    loop {
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut buf)
            .ok()
            .expect("read_line error");
        n = buf.trim().parse().unwrap();
        if n > 1 {
            break;
        } else {
            println!("n must be higher than 1");
        }
    }
    n
}
