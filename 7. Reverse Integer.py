class Solution:
    def reverse(self, x: int) -> int:
        if x == 0:
            return 0
        if x < 0:
            digits = str(x)[1:].rstrip('0')

            res = int('-' + digits[::-1])
        else:
            digits = str(x).rstrip('0')
            res = int(digits[::-1])
        if res > 2 ** 31 - 1 or res <= -2 ** 31:
            return 0
        return res