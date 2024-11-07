use std::collections::HashMap;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut days = vec![0; n];
        let mut stack = vec![]; // This stack will store indices of the temperatures

        for i in 0..n {
            // While there are indices in the stack and the current temperature is greater
            // than the temperature at the index stored at the top of the stack
            while let Some(&j) = stack.last() {
                if temperatures[i] > temperatures[j] {
                    days[j] = (i - j) as i32; // Calculate the number of days
                    stack.pop(); // Remove the index from the stack
                } else {
                    break; // No warmer day found for this index
                }
            }
            stack.push(i); // Push the current index onto the stack
        }

        days
    }
}
