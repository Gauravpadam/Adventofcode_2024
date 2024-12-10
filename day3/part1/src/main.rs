use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn mul(foo: i32, bar: i32) -> Option<i32> {
    foo.checked_mul(bar)
}

fn main() {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found.");
    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Error reading the file.");

    let mut results: Vec<&str> = re.find_iter(&input).map(|c| c.as_str()).collect();
    let mut sum = 0;

    for result in results.iter() {
        let nums = result
            .strip_suffix(")")
            .unwrap()
            .strip_prefix("mul(")
            .unwrap();
        let numbers: Vec<&str> = nums.split(',').map(str::trim).collect();

        println!("{:?}", numbers);

        if numbers.len() >= 2 {
            let _numbers: Vec<i32> = numbers
                .into_iter()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            println!("{}{}", _numbers[0], _numbers[1]);
            if _numbers.len() >= 2 {
                sum += _numbers[0] * _numbers[1];
            }
        }
    }

    println!("{}", sum);
}
