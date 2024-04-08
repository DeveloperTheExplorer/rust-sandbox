use std::collections::HashMap;

use rand::{thread_rng, Rng};
use sandbox::department_sorter::department_sorter;

fn main() {
    department_sorter::runner();
}

// println!("{k} elements removed, nums: {:?}", nums);
// let w1 = "first";
// let w2 = "apple";
// let p1 = "Hello World";
// let p2 = "Sally bought and ate an apple";

// println!("{w1} is {}", pig_latin(w1));
// println!("{w2} is {}", pig_latin(w2));
// println!("{p1} is {}", pig_latin(p1));
// println!("{p2} is {}", pig_latin(p2));
fn pig_latin(phrase: &str) -> String {
    const VOWERLS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let pig_array: Vec<String> = phrase
        .split_whitespace()
        .into_iter()
        .map(|word| -> String {
            let f_char = word.chars().nth(0).unwrap();

            return match VOWERLS.contains(&f_char) {
                true => {
                    format!("{word}-hay")
                }
                false => {
                    format!(
                        "{}-{}ay",
                        word.get(1..).unwrap(),
                        word.get(0..1).unwrap().to_lowercase()
                    )
                }
            };
        })
        .collect();

    return pig_array.join(" ");
}

fn generate_random_vec(&len: &u8) -> Vec<u8> {
    let mut new_vec: Vec<u8> = vec![];

    for _ in 0..len {
        new_vec.push(thread_rng().gen_range(0..10));
    }

    return new_vec;
}

// let len = 20;
// let hard_data = generate_random_vec(&len);
// let ez_way: Vec<u8> = (0..len).map(|_| thread_rng().gen_range(0..10)).collect();

// let (hd_med, hd_mode, hard_data) = get_med_and_mode(hard_data);
// let (ew_med, ew_mode, ez_way) = get_med_and_mode(ez_way);

// println!(
//     "hard way median: {} | mode: {:?} | {:?}",
//     hd_med, hd_mode, hard_data
// );
// println!(
//     "hard way median: {} | mode: {:?} | {:?}",
//     ew_med, ew_mode, ez_way
// );

fn get_med_and_mode(data: Vec<u8>) -> (u8, Option<u8>, Vec<u8>) {
    // let mut sorted_data = data.to_owned();
    let mut sorted_data = data;
    sorted_data.sort();

    let mut mode_hash: HashMap<u8, u8> = HashMap::new();

    for num in &sorted_data {
        let count = mode_hash.entry(*num).or_insert(0);
        *count += 1;
    }

    let mode = mode_hash
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| *num);
    let median = sorted_data[sorted_data.len() / 2];

    return (median, mode, sorted_data);
}

// fn str_parser(value: &mut str) -> &str {
//     value + value
// }

// fn word_by_index(sentence: &str, word_index: u8) -> &str {
//     let str_bytes = sentence.as_bytes();
//     let mut curr_word_index: u8 = 0;
//     let mut word_start_index: usize = 0;

//     for (i, &letter) in str_bytes.iter().enumerate() {
//         if letter == b' ' {
//             if curr_word_index == word_index {
//                 return &sentence[word_start_index..i];
//             }
//             curr_word_index += 1;
//             word_start_index = i + 1;
//         }
//     }

//     return &sentence[word_start_index..];
// }
