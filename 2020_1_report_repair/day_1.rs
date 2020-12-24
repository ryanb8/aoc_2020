use std::fs;
use std::io;
use std::convert::TryInto;
use std::collections::HashMap;

// Assumptions:
// - Always exactly one matching pair in data
// - no empty new lines in data file

const DESIRED_SUM : i16 = 2020;

fn string_from_file() -> Result<String, io::Error>{
    let s = fs::read_to_string("data.txt")?;
    Ok(s)
}

fn load_file_to_ht(ht: &mut HashMap<i16, i16>, s: &str) -> Vec<i16> {
    let mut keys: Vec<i16> = vec![];
    for sub_s in s.trim().split('\n'){
        // let myint = sub_s.parse::<i16>().unwrap();
        keys.push(sub_s.parse::<i16>().unwrap());
        // ht.insert(myint, DESIRED_SUM - myint);
    }
    keys.sort();
    for (ix, key) in keys.iter().enumerate() {
        ht.insert(*key, ix.try_into().unwrap());
    }
    keys
}

fn scan_ht_for_desired_multiple(ht: &HashMap<i16, i16>) -> i32 {
    // O(1)
    for k in ht.keys() {
        if ht.contains_key(&(DESIRED_SUM - k)){
            return *k as i32 * (DESIRED_SUM as i32 - *k as i32);
        }
    }
    panic!("no match found for part 1");
}

fn scan_ht_for_tri_mult(ht: HashMap<i16, i16>, sorted_keys: &Vec<i16> ) -> i32 {
    // I did some googling and I think recursive backtracking would be the best
    // approach here. I'm pointing at that, but not actually doing it.
    //
    // I think we can get to O(n^2) or O(nlogn)
    // Its been a long time since I calculated O() where there are logs in it...
    for (ix, k) in sorted_keys.iter().enumerate() {
        if k >= &DESIRED_SUM{
            break;
        } else {
            let sub_sum = DESIRED_SUM - k;
            // Only search remaining keys
            for k2 in sorted_keys[ix+1..].iter(){
                if k2 >= &sub_sum{
                    break;
                }
                if ht.contains_key(&(sub_sum - k2)){
                    return *k as i32 * *k2 as i32 * (sub_sum as i32 - *k2 as i32);
                }
            }
        }
    }
    panic!("No combination found for part 2");
}

fn main(){
    let s = string_from_file().unwrap();
    let mut ht : HashMap<i16, i16> = HashMap::new();
    let sorted_keys = load_file_to_ht(&mut ht, &s);
    let mult = scan_ht_for_desired_multiple(&ht);
    println!("part 1: {}", mult);

    let mult2 = scan_ht_for_tri_mult(ht, &sorted_keys);
    println!("part 2: {}", mult2);

}

