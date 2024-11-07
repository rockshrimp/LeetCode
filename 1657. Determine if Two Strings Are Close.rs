use std::collections::HashMap;

fn create_char_counter(str: String) -> HashMap<char, i32>{
    let mut char_counter = HashMap::new();
    for ch in str.chars(){
        match char_counter.get(&ch){
            Some(val) => char_counter.insert(ch, val + 1),
            None => char_counter.insert(ch, 1),
        };
    }
    return char_counter;
}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut word1_map = create_char_counter(word1);
        let mut word2_map = create_char_counter(word2);

        let mut word1_char_count = word1_map.values().cloned().collect::<Vec<i32>>();
        word1_char_count.sort();
        let mut word1_chars = word1_map.keys().cloned().collect::<Vec<char>>();
        word1_chars.sort();

        let mut word2_char_count = word2_map.values().cloned().collect::<Vec<i32>>();
        word2_char_count.sort();
        let mut word2_chars = word2_map.keys().cloned().collect::<Vec<char>>();
        word2_chars.sort();
        
        return word1_char_count == word2_char_count && word1_chars == word2_chars;
    }
}