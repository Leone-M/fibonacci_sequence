fn main() {
    let y: i32 = nolan(5);
    println!("{y}");
}

fn nolan(n: i32) -> i32{
    if (n == 1) || (n == 2) {
        return 1
    } else {
        return nolan(n-1) + nolan(n-2)
    }
}