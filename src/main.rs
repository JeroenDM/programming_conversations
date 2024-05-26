#![feature(iter_map_windows)]
use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(PartialEq, Hash)]
struct BiGram {
    w1: String,
    w2: String,
}

impl Eq for BiGram {}

fn main() {
    let content = fs::read_to_string("data/pandp.txt").expect("Failed to read book.");
    println!("Read {} characters.", content.len());

    let mut bigrams = HashMap::new();
    // let _iter: Vec<_> = content
    //     .split_ascii_whitespace()
    //     // .take(10000)
    //     .map_windows(|[x, y]| {
    //         let bg = BiGram {
    //             w1: x.to_string(),
    //             w2: y.to_string(),
    //         };
    //         bigrams.entry(bg).and_modify(|x| *x += 1).or_insert(1);
    //     })
    //     .collect();

    for (x, y) in content.split_ascii_whitespace().into_iter().tuple_windows() {
        bigrams
            .entry(BiGram {
                w1: x.to_string(),
                w2: y.to_string(),
            })
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut sorted = bigrams.into_iter().collect::<Vec<_>>();
    sorted.sort_by(|x, y| y.1.cmp(&x.1));

    println!("Found {} bigrams.", sorted.len());
    for (k, v) in sorted.iter().take(10) {
        println!("{: <18} {}", format!("{}-{} :", k.w1, k.w2), v);
    }
}
