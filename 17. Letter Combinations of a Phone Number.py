class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if digits == "":
            return []
        d = {'2': 'abc', '3': 'def', '4': 'ghi', '5': 'jkl', '6': 'mno', '7': 'pqrs', '8': 'tuv', '9': 'wxyz'}

        res = [""]
        for val in digits:
            letter_count = len(d[val])
            prev_size = len(res) if len(res) > 0 else 1
            res = letter_count * res
            for i in range(0, len(res)):
                res[i] += d[val][i // prev_size]
        return res  