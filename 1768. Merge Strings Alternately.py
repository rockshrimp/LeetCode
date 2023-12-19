class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        length = min(len(word1), len(word2))
        res = []
        for i in range(length):
            res.append(word1[i] + word2[i])

        if length == len(word1):
            res.append(word2[len(word1):])
        else:
            res.append(word1[len(word2):])
        return ''.join(res)