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

fn unique_copy_with_count<T>(v: Vec<T>) -> Vec<(usize, T)>
where
    T: PartialEq + Clone
{
    let mut res = Vec::new();
    if v.len() == 0 {
        return res;
    }
    let mut i = 0;
    let mut n = 0;
    let mut current = &v[i];

    res.push((1, v[0].clone()));
    i += 1;

    while i < v.len() {
        if v[i] != *current {
            res.push((1, v[i].clone()));
            n += 1;
            current = &v[i]
        } else {
            let p = &res[n];
            res[n] = (p.0 + 1, p.1.clone());
        }
        i += 1;
    }
    return res;
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
    // let cnt: HashMap<BiGram, usize> = bigrams.iter().copied().counts();
    let cnt = unique_copy_with_count(bigrams);

    let mut sorted = cnt.into_iter().collect::<Vec<_>>();
    sorted.sort_by(|x, y| y.0.cmp(&x.0));

    for (key, value) in sorted.iter().take(10) {
        println!("{: <20} {}", format!("{}:", &value), &key);
    }

    //
    // let a = vec![1, 2, 3, 4, 5];
    // for (x, y) in a.iter().tuple_windows() {
    //     println!("{} -- {}", x, y);
    // }
}

#[cfg(test)]
mod tests {
    use crate::unique_copy_with_count;


    #[test]
    fn test_unique_1() {

        let input = vec![1, 1, 2, 3, 4, 4, 4];
        let res = unique_copy_with_count(input);
        assert_eq!(res, vec![(2, 1), (1, 2), (1, 3), (3, 4)]);
        
    }
}