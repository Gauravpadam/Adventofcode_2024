use std::fs::File;
use std::io::Read;
use std::path::Path;

fn is_safe(report: &Vec<i32>) -> bool {
    let mut incf = true;
    let mut decf = true;
    let mut df = true;

    for i in 0..report.len() - 1 {
        let diff = (report[i] - report[i + 1]).abs();
        if report[i] > report[i + 1] {
            incf = false;
        }
        if report[i] < report[i + 1] {
            decf = false;
        }
        if diff < 1 || diff > 3 {
            df = false;
        }
        if !(incf || decf) || !df {
            return false;
        }
    }
    true
}

fn remove_and_check(report: &Vec<i32>, ind: usize) -> bool {
    let mut temp = report.clone();
    temp.remove(ind);
    is_safe(&temp)
}

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found!");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read file");

    let processed_lists: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Invalid number"))
                .collect()
        })
        .collect();

    let mut scount = 0;
    let mut nscount = 0;

    for report in &processed_lists {
        if is_safe(report) {
            scount += 1;
        } else {
            let mut is_near_safe = false;
            for i in 0..report.len() {
                if remove_and_check(report, i) {
                    is_near_safe = true;
                    break;
                }
            }
            if is_near_safe {
                nscount += 1;
            }
        }
    }

    println!("Safe count: {}", scount);
    println!("Near-safe count: {}", nscount);
}
