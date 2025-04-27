use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    print!("ハフマン符号にしたい文字列を英小文字か平仮名で入力してください：");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let source = input.trim().to_string();

    let mut counter = HashMap::new();
    for word in source.chars() {
        *counter.entry(word).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (key, value) in &counter {
        heap.push(Reverse((*value, key.to_string())));
    }

    let mut huffman_code = HashMap::new();
    for (key, _) in &counter {
        huffman_code.insert(key, String::new());
    }

    while heap.len() > 1 {
        let Reverse((value1, key1)) = heap.pop().unwrap();
        let Reverse((value2, key2)) = heap.pop().unwrap();
        let new_key = format!("{}{}", key1, key2);
        let new_value = value1 + value2;
        heap.push(Reverse((new_value, new_key.clone())));

        for c in key1.chars() {
            huffman_code.get_mut(&c).map(|code| {
                code.insert(0, '0');
            });
        }
        for c in key2.chars() {
            huffman_code.get_mut(&c).map(|code| {
                code.insert(0, '1');
            });
        }
    }

    println!("---ハフマン符号の対応表---");
    for (key, code) in &huffman_code {
        println!("{}: {}", key, code);
    }
    println!("--------------------------");

    let mut encoded = String::new();
    for c in source.chars() {
        encoded.push_str(&huffman_code[&c]);
    }
    println!("ハフマン符号化された文字列: {}", encoded);
}
