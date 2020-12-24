use std::fs;
use std::io;
use std::collections::HashMap;

// Assumptions:
// - Always exactly one matching pair in data
// - no empty new lines in data file

const DESIRED_SUM : i16 = 2020;

fn string_from_file() -> Result<String, io::Error>{
    let s = fs::read_to_string("data.txt")?;
    Ok(s)
}

fn load_file_to_ht(ht: &mut HashMap<i16, i16>, s: &str) {
    for sub_s in s.trim().split('\n'){
        let myint = sub_s.parse::<i16>().unwrap();
        ht.insert(myint, DESIRED_SUM - myint);
    }
}

fn scan_ht_for_desired_multiple(ht: HashMap<i16, i16>) -> i32 {
    for (k, v) in ht.iter() {
        if ht.contains_key(&v){
            return *k as i32 * *v as i32;
        }
    }
    panic!("no match found");
}

fn main(){
    let s = string_from_file().unwrap();
    let mut ht : HashMap<i16, i16> = HashMap::new();
    load_file_to_ht(&mut ht, &s);
    let mult = scan_ht_for_desired_multiple(ht);
    println!("{}", mult)
}

