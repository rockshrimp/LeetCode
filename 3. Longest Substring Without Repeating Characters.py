class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        visited_char = set()
        i = 0
        j = 0
        max_length = 0
        while j < len(s):
            c = s[j]
            if c not in visited_char:
                 visited_char.add(s[j])
                 max_length = max(max_length, len(visited_char))
            else:
                while s[i] != c:
                    visited_char.remove(s[i])
                    i += 1
                i += 1
            j += 1
        return max_length