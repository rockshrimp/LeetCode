class Solution:
    def kidsWithCandies(self, candies: List[int], extraCandies: int) -> List[bool]:
        max_candies = max(candies)
        return [candies[i] + extraCandies >= max(candies) for i in range(len(candies))]