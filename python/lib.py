import heapq

def get_input():
    print("ハフマン符号化したい文字列を英小文字か平仮名で入力してください: ", end="")
    return input().strip()

def get_huffman_code(counter):
    heap = []
    for key, value in counter.items():
        heapq.heappush(heap, (value, key))

    huffman_code = dict()
    for key in counter.keys():
        huffman_code[key] = []

    if len(heap) == 1:
        _value, key = heapq.heappop(heap)
        huffman_code[key] = "0"
        return huffman_code

    while len(heap) > 1:
        value1, key1 = heapq.heappop(heap)
        value2, key2 = heapq.heappop(heap)
        new_key = key1 + key2
        new_value = value1 + value2
        heapq.heappush(heap, (new_value, new_key))

        for c in key1:
            huffman_code[c].append("0")

        for c in key2:
            huffman_code[c].append("1")

    for key in counter.keys():
        huffman_code[key] = "".join(huffman_code[key])

    return huffman_code

def print_huffman_code(huffman_code):
    print("---ハフマン符号の対応表---")
    for key, value in huffman_code.items():
        print(f"{key}: {value}")
    print("--------------------------")

def get_encoded_huffman_code(huffman_code, source):
    encoded = []
    for c in source:
        encoded.append(huffman_code[c])
    return "".join(encoded)

def print_source_and_encoded_huffman_code(source, encoded_huffman_code):
    print("ハフマン符号化する前の文字列:", source)
    print("ハフマン符号化した後の文字列:", encoded_huffman_code)