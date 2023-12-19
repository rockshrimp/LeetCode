class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if target > nums[-1]:
            return len(nums)
        elif target < nums[0]:
            return 0

        left = 0
        right = len(nums) - 1

        while right - left > 1:
            if nums[left] == target:
                return left
            elif nums[right] == target:
                return right

            center = (right + left) // 2
            if nums[center] == target:
                return center
            elif nums[center] < target:
                left = center
            elif nums[center] > target:
                right = center
        return right