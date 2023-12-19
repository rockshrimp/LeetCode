class Solution:
    def isValid(self, s: str) -> bool:
        d = {'(': ')', '[': ']', '{': '}'}
        queue = []
        for c in s:
            if c in d:
                queue.append(c)
            else:
                if len(queue) > 0 and c == d[queue[-1]]:
                    queue.pop()
                else:
                    return False
        return not len(queue)
