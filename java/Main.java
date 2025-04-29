import java.util.*;

public class Main {
    public static void main(String[] args) {
        String source = getInput();
        Map<Character, Integer> counter = getCounter(source);

        Map<Character, String> huffmanCode = getHuffmanCode(counter);
        printHuffmanCode(huffmanCode);

        String encodedHuffmanCode = getEncodedHuffmanCode(huffmanCode, source);
        printSourceAndEncodedHuffmanCode(source, encodedHuffmanCode);
    }

    public static String getInput() {
        System.out.print("ハフマン符号化したい文字列を英小文字か平仮名で入力してください: ");

        Scanner sc = new Scanner(System.in);
        String input = sc.next().trim();
        sc.close();
        return input;
    }

    public static Map<Character, Integer> getCounter(String source) {
        Map<Character, Integer> counter = new HashMap<>();
        for (char word : source.toCharArray()) {
            counter.put(word, counter.getOrDefault(word, 0) + 1);
        }
        return counter;
    }

    public static Map<Character, String> getHuffmanCode(Map<Character, Integer> counter) {
        Queue<Map<Integer, String>> heap = new PriorityQueue<>(
                (x, y) -> {
                    Integer keyX = x.keySet().iterator().next();
                    Integer keyY = y.keySet().iterator().next();
                    return Integer.compare(keyX, keyY);
                });
        for (Map.Entry<Character, Integer> entry : counter.entrySet()) {
            Map<Integer, String> ele = new HashMap<>();
            ele.put(entry.getValue(), String.valueOf(entry.getKey()));
            heap.add(ele);
        }

        HashMap<Character, String> huffmanCode = new HashMap<>();
        for (Map.Entry<Character, Integer> entry : counter.entrySet()) {
            huffmanCode.put(entry.getKey(), "");
        }

        if (heap.size() == 1) {
            Map<Integer, String> ele = heap.poll();
            Integer value = ele.keySet().iterator().next();
            String key = ele.get(value);
            huffmanCode.put(key.charAt(0), "0");
        }

        while (heap.size() > 1) {
            Map<Integer, String> ele1 = heap.poll();
            Map<Integer, String> ele2 = heap.poll();
            Integer value1 = ele1.keySet().iterator().next();
            String key1 = ele1.get(value1);
            Integer value2 = ele2.keySet().iterator().next();
            String key2 = ele2.get(value2);

            String newKey = key1 + key2;
            Integer newValue = value1 + value2;
            heap.add(Map.of(newValue, newKey));

            for (char c : key1.toCharArray()) {
                huffmanCode.put(c, "0" + huffmanCode.get(c));
            }
            for (char c : key2.toCharArray()) {
                huffmanCode.put(c, "1" + huffmanCode.get(c));
            }
        }
        return huffmanCode;
    }

    public static void printHuffmanCode(Map<Character, String> huffmanCode) {
        System.out.println("---ハフマン符号の対応表---");
        for (Map.Entry<Character, String> entry : huffmanCode.entrySet()) {
            System.out.println(entry.getKey() + " : " + entry.getValue());
        }
        System.out.println("--------------------------");
    }

    public static String getEncodedHuffmanCode(Map<Character, String> huffmanCode, String source) {
        StringBuilder encoded = new StringBuilder();
        for (char c : source.toCharArray()) {
            encoded.append(huffmanCode.get(c));
        }
        return encoded.toString();
    }

    public static void printSourceAndEncodedHuffmanCode(String source, String encodedHuffmanCode) {
        System.out.println("ハフマン符号化する前の文字列: " + source);
        System.out.println("ハフマン符号化した後の文字列: " + encodedHuffmanCode);
    }
}