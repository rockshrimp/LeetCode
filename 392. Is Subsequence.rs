impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.chars();
        let mut t_iter = t.chars();

        if s.len() == 0{
          return true;  
        }

        let mut s_char = s_iter.next().unwrap();
        let mut t_char;
        while true{
             match t_iter.next() {
                    Some(p) => {
                        t_char = p;
                    },
                    None => return false
            }
            if s_char == t_char {
                match s_iter.next() {
                    Some(p) => {
                        s_char = p;
                    },
                    None => return true
                }
            }            
        }

        return false;
    }
}