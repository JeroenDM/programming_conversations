#![feature(iter_map_windows)]
#![feature(hash_set_entry)]

use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash)]
struct BiGram {
    w1: String,
    w2: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct MyPair<'a> {
    s1: &'a String,
    s2: &'a String,
}

fn bigram_v1(filename: &str) {
    let content = fs::read_to_string(filename).expect("Failed to read book.");
    println!("Read {} characters.", content.len());

    let mut bigrams = HashMap::new();

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

fn bigram_with_extra_set(filename: &str) {
    let content = fs::read_to_string(filename).expect("Failed to read book.");
    println!("Read {} characters.", content.len());

    let mut words = HashSet::new();
    // Helper scope to prove a point later on.
    {
        let mut bigrams = HashMap::new();

        // Split this up into two loops,
        // once we put x_ref into the map,
        // we cannot borrow words mutable anymore because
        // the reference is around as long as 'bigrams' is around.
        for (x, y) in content.split_ascii_whitespace().into_iter().tuple_windows()
        // .take(5)
        {
            words.insert(x.to_string());
            words.insert(y.to_string());
        }

        for (x, y) in content.split_ascii_whitespace().into_iter().tuple_windows()
        // .take(5)
        {
            let x_ref = words.get(x).unwrap();
            let y_ref = words.get(y).unwrap();

            bigrams
                .entry(MyPair {
                    s1: x_ref,
                    s2: y_ref,
                })
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        // This does not work anymore
        // words.insert("hello".to_string());

        let mut sorted = bigrams.into_iter().collect::<Vec<_>>();
        sorted.sort_by(|x, y| y.1.cmp(&x.1));

        println!("Found {} bigrams.", sorted.len());
        for (k, v) in sorted.iter().take(10) {
            println!("{: <18} {}", format!("{}-{} :", k.s1, k.s2), v);
        }
    }

    // This does work because 'bigrams', and therefore all references
    // to words, are out of scope.
    let inserted = words.insert("hello".to_string());
    println!("hello in words?: {}", inserted);
}

fn main() {
    // bigram_v1("data/pandp.txt");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: binary <filename>")
    } else {
        bigram_v1(&args[1]);
        bigram_with_extra_set(&args[1]);
    }
}
