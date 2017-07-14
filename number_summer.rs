use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
                
fn main() {   
    let f = File::open("nums.txt");
    let mut file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l[..15]); 
    }           
}