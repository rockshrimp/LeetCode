import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

class Solution {
    public int majorityElement(int[] nums) {
        HashMap<Integer, Integer> counter = new HashMap<Integer, Integer>();
        for(int i = 0; i < nums.length; i++){
            if (!counter.containsKey(nums[i])){
                counter.put(nums[i], 1);
            }else{
                counter.put(nums[i], counter.get(nums[i]) + 1);
            }

            counter.forEach((key, value) -> System.out.println(key + " " + value));
        }
        return Collections.max(counter.entrySet(), Map.Entry.comparingByValue()).getKey();
    }
}