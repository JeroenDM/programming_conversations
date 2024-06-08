use memmap2::MmapOptions;
use std::fs::File;

fn print_ascii_array(array: &[u8]) {
    for &byte in array.iter() {
        print!("{}", byte as char);
    }
    println!();
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("data/pandp.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    println!("#bytes in file: {:?}", mmap.len());
    // println!("{:?}", &mmap[0..10]);

    let mut current: [u8; 8] = [b' '; 8];
    let mut i = 0;
    // let a: Vec<_> = mmap.iter().map(|x| *x as char).collect();

    let mut words: Vec<[u8; 8]> = Vec::new();
    for x in mmap.iter().take(100) {
        if  (*x as char).is_whitespace() {
            words.push(current);
            i = 0;
        }
        else if i > 7 {
            // skip byte
        }
        else {

            current[i] = *x;
            i += 1;
        }
    }

    for word in words {
        print_ascii_array(&word);
    }

    Ok(())
}
