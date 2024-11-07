class Solution {
    public void merge(int[] nums1, int m, int[] nums2, int n) {
        if (n == 0){
            return;
        }
        int n1_idx = m - 1;
        int n2_idx = n - 1;

        for(int i = m + n - 1; i >= 0; i--){
            if (n1_idx < 0 || n2_idx < 0){
                if (n1_idx < 0){
                    nums1[i] = nums2[n2_idx];
                    n2_idx--;
                }
                else{
                    nums1[i] = nums1[n1_idx];
                    n1_idx--;
                }
            } else {
                if (nums1[n1_idx] > nums2[n2_idx]){
                    nums1[i] = nums1[n1_idx];
                    n1_idx--;
                }else{
                    nums1[i] = nums2[n2_idx];
                    n2_idx--;
                }
            }
        }
    }
}