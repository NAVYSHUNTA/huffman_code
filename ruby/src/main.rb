require_relative 'lib'

source = get_input()
counter = get_counter(source)

huffman_code = get_huffman_code(counter)
print_huffman_code(huffman_code)

encoded_huffman_code = get_encoded_huffman_code(huffman_code, source)
print_source_and_encoded_huffman_code(source, encoded_huffman_code)
