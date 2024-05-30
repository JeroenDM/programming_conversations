use std::mem::size_of;


fn main(){

    // let word = "codeberg";
let a : u16 = 0b110110011001100;
let b = a << 8;

println!("{:b}", a);
println!("{:b}", b);
println!("{}", size_of::<u32>());
println!("{}", size_of::<[u8;8]>());
}