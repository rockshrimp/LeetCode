import java.util.HashMap;

class Solution {
    public static boolean wordPattern(String pattern, String s) {
        String[] words = s.split(" ");
        HashMap<Character, String> charToWord = new HashMap<>();
        HashMap<String, Character> wordToChar = new HashMap<>();
        if (pattern.length() != words.length){
            return false;
        }
        for (int i = 0; i<words.length; i++){
            String word = words[i];
            if (wordToChar.containsKey(word) && !wordToChar.get(word).equals(pattern.charAt(i))) {
                return false;
            }

            if (!charToWord.containsKey(pattern.charAt(i))){
                charToWord.put(pattern.charAt(i), word);
                wordToChar.put(word, pattern.charAt(i));
            }
            else
            {
                if (!charToWord.get(pattern.charAt(i)).equals(word)){
                    System.out.println(word + " " +pattern.charAt(i) + " " + charToWord.get(pattern.charAt(i)));
                    return false;
                }     
            }
        }
        return true;
    }
}