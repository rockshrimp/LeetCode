class Solution:
    def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
        return set(a for a in nums1 if a in nums2)