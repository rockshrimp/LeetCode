class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        common_prefix = ""
        min_length = min(len(s) for s in strs)
        for i in range(min_length):
            if all(strs[0][i] == strs[j][i] for j in range(1, len(strs))):
                common_prefix += strs[0][i]
            else:
                return common_prefix
        return common_prefix

