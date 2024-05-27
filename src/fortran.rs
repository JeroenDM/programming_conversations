use std::collections::HashMap;
use std::fmt;
use std::fs;

use itertools::Itertools;

const IGNORE: char = '\0';
const CHUNK_SIZE: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Chunk<const SIZE: usize> {
    data: [char; SIZE],
}

impl<const S: usize> Chunk<S> {
    fn from(x: &str) -> Self {
        let mut chunk = Chunk { data: [IGNORE; S] };
        for (i, c) in x.chars().take(S).enumerate() {
            chunk.data[i] = c;
        }
        chunk
    }
}

impl<const S: usize> fmt::Display for Chunk<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Is there a better way?
        let is_visible = |x: &char| *x != IGNORE;
        let s = String::from_iter(self.data.into_iter().filter(is_visible));
        write!(f, "{}", s)
    }
}

// impl<const S: usize> FromIterator<char> for Chunk<S> {
//     fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
//         let mut data: [char; S] = [IGNORE; S];
//         for (i, c) in iter.into_iter().take(S).enumerate() {
//             data[i] = c;
//         }
//         Chunk { data }
//     }
// }

type Chunk8 = Chunk<CHUNK_SIZE>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BiGram {
    w1: Chunk8,
    w2: Chunk8,
}

impl BiGram {
    fn from(t: (Chunk8, Chunk8)) -> Self {
        Self { w1: t.0, w2: t.1 }
    }
}

impl fmt::Display for BiGram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.w1, self.w2)
    }
}

fn main() {
    let filename = "data/pandp.txt";
    let content = fs::read_to_string(filename).expect("Failed to read book.");
    // println!("Read {} characters.", content.len());

    let mut bigrams: Vec<BiGram> = content
        .split_whitespace()
        .map(|x| Chunk8::from(x))
        .tuple_windows()
        .map(|t| BiGram::from(t))
        // .take(100)
        .collect();

    bigrams.sort();

    // for &bg in bigrams.iter() {
    //     println!("{}", bg);
    // }

    // replace this with algorithm, not hashmap
    let cnt : HashMap::<BiGram, usize> = bigrams.iter().copied().counts(); 

    let mut sorted = cnt.into_iter().collect::<Vec<_>>();
    sorted.sort_by(|x, y| y.1.cmp(&x.1));

    for (key, value) in sorted.iter().take(10) {
        println!("{: <20} {}", format!("{}:", &key), &value);
    }

    //
    // let a = vec![1, 2, 3, 4, 5];
    // for (x, y) in a.iter().tuple_windows() {
    //     println!("{} -- {}", x, y);
    // }
}
