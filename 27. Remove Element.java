class Solution {
    public int removeElement(int[] nums, int val) {
        if (nums.length == 1 && nums[0] == val){
            return 0;
        }
        int val_insert_idx = nums.length - 1;
        int val_count = 0;
        int i = 0;
        while (i <= val_insert_idx){
            System.out.println(i + " " + val_insert_idx);
            if (nums[i] == val) {
                System.out.println("Found val at idx :" + i);
                val_count++;
                if (nums[val_insert_idx] != val) {
                    int temp = nums[val_insert_idx];
                    nums[val_insert_idx] = nums[i];
                    nums[i] = temp;
                    val_insert_idx--;
                }
                else{
                    while(nums[val_insert_idx] == val && val_insert_idx > i){
                        val_insert_idx--;
                        val_count++;
                        System.out.println("Found val at index : " + val_insert_idx);
                    }

                    int temp = nums[val_insert_idx];
                    nums[val_insert_idx] = nums[i];
                    nums[i] = temp;
                    val_insert_idx--;


                }
            }
            i++;
        }
        System.out.println(i + " " + val_insert_idx);
        System.out.println(Arrays.toString(nums));
        System.out.println(val_count);

        return nums.length - val_count;
    }
}