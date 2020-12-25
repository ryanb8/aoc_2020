use std::fs;
use std::collections::HashMap;
use std::convert::TryFrom;

//Playing around with structs...
//Not a great use case.
struct NestedHm {
    hm: HashMap<i16, HashMap<i16, bool>>
}

impl Default for NestedHm {
    fn default() -> Self{
        NestedHm {
            hm: HashMap::new()
        }
    }
}

impl NestedHm {
    fn new() -> Self {
        Default::default()
    }
    fn num_rows(&self) -> i16 {
        self.hm.len() as i16
    }
    fn num_cols(&self) -> i16 {
        // Rust doesn't have null
        // So I'm not 100% sure what the rust-like way to handle this is
        // - Raise an error if there's no hash table?
        // - Return an enum (still learning about that)?
        self.hm.get(&0i16).unwrap().len() as i16
    }
}


fn string_from_file() -> String {
    let s = fs::read_to_string("puzzle_input.txt");
    s.ok().unwrap()
}

fn file_to_hash_nested(ht: &mut NestedHm, s: &str) {
    let mut ix = 0;
    for sub_s in s.trim().split('\n'){
        let mut subh : HashMap<i16, bool> = HashMap::new();
        for (colx, chr) in sub_s.chars().enumerate() {
            if chr == '.' {
                subh.insert(i16::try_from(colx).unwrap(), false);
            } else if chr == '#' {
                // subh.insert(i16::try_from(colx).ok(), true);
                subh.insert(i16::try_from(colx).unwrap(), true);
            } else {
                panic!("All characters must be . or #")
            }
        }
        ht.hm.insert(ix, subh);
        ix = ix + 1;
    }
}

fn count_trees(right: i16, down: i16, ht: &NestedHm) -> i16 {
    let mut h : i16 = 0;
    let mut v : i16 = 0;
    let nrow = ht.num_rows();
    let ncol = ht.num_cols();
    let mut num_trees = 0i16;

    if *ht.hm.get(&0i16).unwrap().get(&0i16).unwrap() {
        num_trees = num_trees + 1;
    }

    while nrow - 1 > v {
        h = h + right;
        v = v + down;
        if h >= ncol {
            h = h - ncol;
        }
        // println!("{}", *ht.hm.get(&v).unwrap().get(&h).unwrap());
        // println!("row {}", &v);
        if *ht.hm.get(&v).unwrap().get(&h).unwrap() {
            num_trees = num_trees + 1;
        }
    }
    num_trees
}

fn main() {
    // let mut ht;
    let mut ht = NestedHm::new();
    let s = string_from_file();
    file_to_hash_nested(&mut ht, &s);
    let num_trees = count_trees(3i16, 1i16, &ht);
    println!("Part 1: {}", num_trees);

    let part_2 =
        count_trees(1i16, 1i16, &ht) as i64 *
        num_trees as i64 *
        count_trees(5i16, 1i16, &ht) as i64 *
        count_trees(7i16, 1i16, &ht) as i64 *
        count_trees(1i16, 2i16, &ht) as i64;
    println!("Part 2: {}", part_2);
}
