class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = []
        nums.sort()

        i = 0
        # for i in range(len(nums) - 2):
        while nums[i] < 1 and i < len(nums) - 2:
            if i > 0 and nums[i] == nums[i - 1]:
                i += 1
                continue

            left = i + 1
            right = len(nums) - 1

            while left < right:
                triplet_sum = nums[i] + nums[left] + nums[right]
                if triplet_sum == 0:
                    res.append([nums[i], nums[left], nums[right]])
                    while left < right and nums[left] == nums[left + 1]:
                        left += 1
                    while left < right and nums[right] == nums[right - 1]:
                        right -= 1
                    left += 1
                    right -= 1
                elif triplet_sum > 0:
                    right -= 1
                else:
                    left += 1
            i += 1
        return res

