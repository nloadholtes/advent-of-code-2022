use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let input = read_input(&args[1])?;
    let lines = input.split("\n\n");

    let mut max = Vec::new();
    for group in lines {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().unwrap();
            // println!("   - {value}");
            sum += value;
        }
        // // println!("Group sum: {sum}");
        // if sum > max {
        //     max = sum;
        // }
        max.push(sum);
    }
    max.sort_by_key(|&v| v);
    println!("{}", max[3]);
    dbg!(max);
    Ok(())
}

fn read_input(input_file: &String) -> Result<String, std::io::Error> {
    // now from the `fs-err` crate, rather than `std::fs`
    fs_err::read_to_string(input_file)
}
