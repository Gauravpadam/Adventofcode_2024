use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;
use std::num;
use std::path::Path;
// Two heaps and it's over.
fn main() {
    let path = Path::new("input.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let result: Vec<i32> = s
        .split_whitespace()
        .map(|strin| strin.parse::<i32>().expect("Merry Christmas!"))
        .collect();

    let mut heap_one = BinaryHeap::new();
    let mut heap_two = BinaryHeap::new();

    for i in 0..result.len() {
        if i % 2 == 0 {
            heap_one.push(result[i].to_owned());
        } else {
            heap_two.push(result[i].to_owned());
        }
    }

    let mut diffsum = 0;

    while let Some(first) = heap_one.pop() {
        diffsum += (first - heap_two.pop().unwrap()).abs();
    }

    println!("{}", diffsum);
}
