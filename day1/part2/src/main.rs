use std::collections::HashMap;
mod modules;
use modules::counter::Counter;
use std::fs::File;
use std::io::Read;
use std::path::Path;
// Two heaps and it's over
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

    let mut first_list = vec![];
    let mut second_list = vec![];

    for (i, el) in result.into_iter().enumerate() {
        if i % 2 == 0 {
            first_list.push(el);
        } else {
            second_list.push(el);
        }
    }

    println!("{:?}", first_list);
    println!("{:?}", second_list);

    let second_map: HashMap<i32, usize> = Counter::from(second_list);

    // println!("{:?}", second_map)

    let mut sim_score = 0;

    for num in first_list {
        // println!(
        //     "First list elem: {}, Second list occurences: {}",
        //     num,
        //     second_map.get(&num).unwrap_or(&0)
        // );
        sim_score += num * *second_map.get(&num).unwrap_or(&0) as i32;
    }

    println!("{}", sim_score);
}
