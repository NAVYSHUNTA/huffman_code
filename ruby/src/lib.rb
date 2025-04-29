# https://github.com/universato/ac-library-rb/blob/main/lib/priority_queue.rb
require 'ac-library-rb/priority_queue'

def get_input
  print "ハフマン符号化したい文字列を英小文字か平仮名で入力してください: "
  gets.strip
end

def get_counter source
  counter = {}
  source.chars.each {|word|
    if counter.key?(word)
      counter[word] += 1
    else
      counter[word] = 1
    end
  }
  counter
end

def get_huffman_code(counter)
  heap = AcLibraryRb::PriorityQueue.new {|x, y| x[0] < y[0]}
  counter.each {|key, value|
    heap.push([value, key])
  }

  huffman_code = {}
  counter.each {|key, _|
    huffman_code[key] = ""
  }

  while heap.size > 1
    value1, key1 = heap.pop
    value2, key2 = heap.pop
    new_key = key1 + key2
    new_value = value1 + value2
    heap.push([new_value, new_key])

    key1.chars.each {|c|
      huffman_code[c] = "0" + huffman_code[c]
    }
    key2.chars.each {|c|
      huffman_code[c] = "1" + huffman_code[c]
    }
  end

  huffman_code
end

def print_huffman_code huffman_code
  puts "---ハフマン符号の対応表---"
  huffman_code.each {|word, code|
    puts "#{word}: #{code}"
  }
  puts "--------------------------"
end

def get_encoded_huffman_code huffman_code, source
  source.chars.map {|c|
    huffman_code[c]
  }.join
end

def print_source_and_encoded_huffman_code source, encoded_huffman_code
  puts "ハフマン符号化する前の文字列: #{source}"
  puts "ハフマン符号化した後の文字列: #{encoded_huffman_code}"
end
