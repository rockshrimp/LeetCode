class Solution {
    public int maxProfit(int[] prices) {
        int maxProfit = 0;
        int min = Integer.MAX_VALUE;
        int max = 0;
        for (int i = 0; i < prices.length; i++){
            int val = prices[i];
            if (val < min){
                min = val; 
                max = 0;
            }
            if (val > max){
                max = val;
                maxProfit = Math.max(max - min, maxProfit); 
            }
        }
        return maxProfit;
    }
}