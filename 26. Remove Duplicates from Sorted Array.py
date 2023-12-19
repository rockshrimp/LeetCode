class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        k = 0
        values = set()
        for i in range(len(nums)):
            if nums[i] not in values:
                values.add(nums[i])
                k += 1
            else:
                nums[i] = 101
        print(k, nums)
        nums.sort()
        return k