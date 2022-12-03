fn main() {
    let input = read_input().unwrap();
    println!("{input}");
}

fn read_input() -> Result<String, std::io::Error> {
    // now from the `fs-err` crate, rather than `std::fs`
    fs_err::read_to_string("src/input.txt")
}
