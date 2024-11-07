impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut current_altitude = 0;

        for g in gain {
            current_altitude += g;
            if current_altitude > max {
                max = current_altitude;
            }
        }
        max
    }
}