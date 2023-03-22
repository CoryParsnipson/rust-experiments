use std::collections::HashMap;
use std::io;

const MAX_NUMBERS: usize = 100;

fn median(numbers: &Vec<i32>) -> i32 {
    let mut sorted_num = numbers.to_vec();
    sorted_num.sort_unstable();

    if sorted_num.len() == 0 {
        return 0;
    }

    if sorted_num.len() % 2 == 0 {
        let l = sorted_num.len();
        (sorted_num[(l / 2) - 1] + sorted_num[l / 2]) / 2
    } else {
        sorted_num[sorted_num.len() / 2]
    }
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut dup = HashMap::new();
    let mut highest = 0;
    let mut modes: Vec<i32> = Vec::new();

    for n in numbers {
        let entry = dup.entry(n).or_insert(0);
        *entry += 1;

        if highest < *entry {
            highest = *entry;
        }
    }

    for (k, v) in dup {
        if v == highest {
            modes.push(*k);
        }
    }

    return modes;
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    while numbers.len() < MAX_NUMBERS {
        let mut new_num = String::new();

        println!("Numbers collected so far:\n{:?}", numbers);
        println!("Please enter in numbers to analyze:");
        io::stdin()
            .read_line(&mut new_num)
            .expect("failed to read line");

        if new_num.trim().is_empty() {
            println!("Received empty input. Number collection complete.");
            break;
        }

        let new_num: i32 = match new_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer (i32).");
                continue;
            }
        };
        numbers.push(new_num);
    }

    println!("Finding median and mode of:\n{:?}", numbers);

    let median = median(&numbers);
    println!("The median is {median}.");

    let mode = mode(&numbers);
    let mut mode: Vec<String> = mode.into_iter().map(|n| n.to_string()).collect();

    if mode.len() > 1 {
        mode.insert(mode.len() - 1, String::from("and"));
    }

    if mode.len() > 1 {
        let last = if let Some(el) = mode.pop() { el } else { String::from("") };
        println!("The modes are {} {}.", &mode.join(", "), &last);
    } else if mode.len() == 1 {
        println!("The mode is {}.", &mode[0]);
    } else {
        println!("There is no mode for empty list.");
    };
}
