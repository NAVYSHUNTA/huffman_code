use rust::*;
use std::collections::HashMap;

#[test]
fn test_get_counter_english() {
    let input = "abcdabc".to_string();
    let expected = HashMap::from([('a', 2), ('b', 2), ('c', 2), ('d', 1)]);
    let result = get_counter(input);
    assert_eq!(result, expected);
}

#[test]
fn test_get_counter_japanese() {
    let input = "あいうえおう".to_string();
    let expected = HashMap::from([('あ', 1), ('い', 1), ('う', 2), ('え', 1), ('お', 1)]);
    let result = get_counter(input);
    assert_eq!(result, expected);
}

#[test]
fn test_get_counter_empty() {
    let input = "".to_string();
    let expected: HashMap<char, usize> = HashMap::new();
    let result = get_counter(input);
    assert_eq!(result, expected);
}

#[test]
fn test_get_counter_all_same() {
    let input = "aaaa".to_string();
    let expected = HashMap::from([('a', 4)]);
    let result = get_counter(input);
    assert_eq!(result, expected);
}

#[test]
fn test_get_encoded_huffman_code() {
    let huffman_code = HashMap::from([
        ('a', "00".to_string()),
        ('b', "01".to_string()),
        ('c', "10".to_string()),
        ('d', "11".to_string()),
    ]);
    let source = "abcdaac".to_string();
    let expected = "00011011000010".to_string();
    let result = get_encoded_huffman_code(huffman_code, source);
    assert_eq!(result, expected);
}

#[test]
fn test_get_encoded_huffman_code_empty() {
    let huffman_code = HashMap::new();
    let source = "".to_string();
    let expected = "".to_string();
    let result = get_encoded_huffman_code(huffman_code, source);
    assert_eq!(result, expected);
}
