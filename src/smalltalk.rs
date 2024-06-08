#![feature(iter_map_windows)]
#![feature(hash_set_entry)]

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct MyPair<'a> {
    s1: &'a String,
    s2: &'a String,
}

fn bigram_with_extra_set(filename: &str) {
    let content = fs::read_to_string(filename).expect("Failed to read book.");
    println!("Read {} characters.", content.len());

    // The approach consists of 4 step:
    // 1. Collect all unique words in a vector.
    // 2. Build a hashmap of all unique bygrams, with there count.
    //    Only store references to the actual strings.
    // 3. Sort in decreasing order of count.
    // 4. Report top 250.

    let mut words = HashSet::new();
    for (x, y) in content.split_whitespace().into_iter().tuple_windows() {
        words.insert(x.to_string());
        words.insert(y.to_string());
    }

    let mut bigrams = HashMap::new();
    for (x, y) in content.split_whitespace().into_iter().tuple_windows() {
        bigrams
            .entry(MyPair {
                s1: words.get(x).unwrap(),
                s2: words.get(y).unwrap(),
            })
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut sorted = bigrams.into_iter().collect::<Vec<_>>();
    sorted.sort_by(|x, y| y.1.cmp(&x.1));

    println!("Found {} bigrams.", sorted.len());
    for (k, v) in sorted.iter().take(10) {
        println!("{: <18} {}", format!("{} {} :", k.s1, k.s2), v);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: binary <filename>")
    } else {
        bigram_with_extra_set(&args[1]);
    }
}
