class Solution {
    public String reverseWords(String s) {
        StringBuilder builder = new StringBuilder();
        String[] words = s.trim().split("\\s+");
        for (int i = words.length - 1; i >= 0; i--){
            builder.append(words[i]);
            if (i != 0){
                builder.append(" ");
            }
        }
        return builder.toString();
    }
}