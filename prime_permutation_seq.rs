// use std::iter::*;

fn main() {
    let foo: String = "abc".to_string();
    let bar: String = "cba".to_string();
    let check = check_if_permutation(foo, bar);
    println!("{}", check);
}


fn check_if_permutation(str1 : String, str2 : String) -> bool {
    let mut chars1: Vec<char> = str1.chars().collect();
    chars1.sort();
    let mut chars2: Vec<char> = str2.chars().collect();
    chars2.sort();
    let string1 = String::new();
    let string2 = String::new();
    for c1 in chars1:
    return chars1.to_string().as_bytes().eq(chars2().to_string().as_bytes());
    
}

fn check_if_prime(num: u32) -> bool {
    let mut ind = 3;
    if num % 2 == 0 {
        return false;
    }
    while ind * ind < num + 1 {
        if num % ind == 0 {
            return false;
        }
        ind += 1;
    }
    return true;
}