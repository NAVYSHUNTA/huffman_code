use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Write};

pub fn get_input() -> String {
    print!("ハフマン符号化したい文字列を英小文字か平仮名で入力してください: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

pub fn get_counter(source: String) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for word in source.chars() {
        *counter.entry(word).or_insert(0) += 1;
    }
    counter
}

pub fn get_huffman_code(counter: HashMap<char, usize>) -> HashMap<char, String> {
    let mut heap = BinaryHeap::new();
    for (key, value) in &counter {
        heap.push(Reverse((*value, key.to_string())));
    }

    let mut huffman_code = HashMap::new();
    for (key, _) in &counter {
        huffman_code.insert(*key, String::new());
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

    huffman_code
}

pub fn print_huffman_code(huffman_code: HashMap<char, String>) {
    println!("---ハフマン符号の対応表---");
    for (word, code) in &huffman_code {
        println!("{}: {}", word, code);
    }
    println!("--------------------------");
}

pub fn get_encoded_huffman_code(huffman_code: HashMap<char, String>, source: String) -> String {
    let mut encoded = String::new();
    for c in source.chars() {
        encoded.push_str(&huffman_code[&c]);
    }
    encoded
}

pub fn print_source_and_encoded_huffman_code(source: String, encoded_huffman_code: String) {
    println!("ハフマン符号化する前の文字列: {}", source);
    println!("ハフマン符号化した後の文字列: {}", encoded_huffman_code);
}
