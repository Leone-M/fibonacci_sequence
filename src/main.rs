use std::io;

fn main() {

    let mut y: String = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read integer");
    println!("{y}");

    let y: i32 = y.trim().parse().expect("Please type a number");

    let x: i32 = nolan(y);
    println!("{y} number of fibonacci sequence is: {x}")
}

fn nolan(n: i32) -> i32{
    if (n == 1) || (n == 2) {
        return 1
    } else {
        return nolan(n-1) + nolan(n-2)
    }
}