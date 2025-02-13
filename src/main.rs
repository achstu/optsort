use std::collections::HashMap;
// use std::collections::HashSet;

use itertools::{Itertools, Unique};

fn solution1() {
    let data = vec!["ala", "ma", "ma", "kota", "ala"];
    let mapping = data.iter().unique().zip(0..).collect::<HashMap<_, _>>();
    let mapped_data = data.iter().map(|word| mapping[word]).collect::<Vec<_>>();

    println!("{:?}", data);
    println!("{:?}", mapping);
    println!("{:?}", mapped_data);
}

fn solution2() {
    let data: Vec<String> = vec!["ala", "ma", "ma", "kota", "ala"]
        .iter()
        .map(|word| word.to_string())
        .collect();
    // let mut mapping = HashMap::new();

    let mapped_data: Vec<usize> = {
        let mut mapping = HashMap::new();
        data.iter()
            .map(|word| {
                let index = mapping.len();
                *mapping.entry(word).or_insert(index)
            })
            .collect()
    };

    println!("{:?}", data);
    // println!("{:?}", mapping);
    println!("{:?}", mapped_data);
}

fn main() {
    // let data = vec![
    //     vec![1, 2, 3],
    //     vec![1, 2],
    //     vec![1, 2, 3],
    //     vec![2, 3],
    //     vec![1, 2],
    //     vec![1, 2, 3],
    // ];

    solution2();
}
