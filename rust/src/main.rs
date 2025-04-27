use rust::*;

fn main() {
    let source = get_input();
    let counter = get_counter(source.clone());

    let huffman_code = get_huffman_code(counter.clone());
    print_huffman_code(huffman_code.clone());

    let encoded_huffman_code = get_encoded_huffman_code(huffman_code, source.clone());
    print_source_and_encoded_huffman_code(source.clone(), encoded_huffman_code.clone());
}
