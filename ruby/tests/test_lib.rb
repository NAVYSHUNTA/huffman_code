require 'minitest/autorun'
require_relative './../src/lib'

class TestLib < Minitest::Test
  def test_get_counter_english
    input = "abcdabc"
    expected = {"a" => 2, "b" => 2, "c" => 2, "d" => 1}
    result = get_counter input
    assert_equal result, expected
  end

  def test_get_counter_japanese
    input = "あいうえおう"
    expected = {"あ" => 1, "い" => 1, "う" => 2, "え" => 1, "お" => 1}
    result = get_counter input
    assert_equal result, expected
  end

  def test_get_counter_empty
    input = ""
    expected = {}
    result = get_counter input
    assert_equal result, expected
  end

  def test_get_counter_all_same
    input = "aaaa"
    expected = {"a" => 4}
    result = get_counter input
    assert_equal result, expected
  end

  def test_get_encoded_huffman_code
    huffman_code = {
      "a" => "00",
      "b" => "01",
      "c" => "10",
      "d" => "11",
    }
    source = "abcdaac"
    expected = "00011011000010"
    result = get_encoded_huffman_code huffman_code, source
    assert_equal result, expected
  end

  def test_get_encoded_huffman_code_empty
    huffman_code = {}
    source = ""
    expected = ""
    result = get_encoded_huffman_code huffman_code, source
    assert_equal result, expected
  end
end