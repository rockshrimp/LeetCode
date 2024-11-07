impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut prev = 0;
        let mut valid_count = 0;
        for i in 0..flowerbed.len(){
            let mut next;
            if i < flowerbed.len() - 1{
                next = flowerbed[i + 1];
            } else {
                next = 0;
            }
            if prev == 0 && flowerbed[i] == 0 && next == 0{
                valid_count += 1;
                prev = 1;
            }else{
                prev = flowerbed[i];
            }
        }
        return valid_count >= n;
    }
}