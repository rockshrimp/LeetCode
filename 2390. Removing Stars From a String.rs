impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut res = Vec::new();
        for c in s.chars(){
            if c != '*'{
                res.push(c);    
            }else{
                res.pop();
            };
        }    
        return res.into_iter().collect();
    }
}