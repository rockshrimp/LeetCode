from math import floor


class Solution:
    image = []
    height = 0
    width = 0

    def smooth_adjacents_cells(self, pos_y: int, pos_x: int) -> int:
        val = 0
        adjacent_cell_count = 0
        for y in range(pos_y - 1, pos_y + 2):
            if y < 0 or y >= self.height:
                continue
            for x in range(pos_x - 1, pos_x + 2):
                if x < 0 or x >= self.width:
                    continue
                else:
                    val += self.image[y][x]
                    adjacent_cell_count += 1
        return int(floor(val / adjacent_cell_count))

    def imageSmoother(self, img: List[List[int]]) -> List[List[int]]:
        self.image = img
        self.width = len(img[0])
        self.height = len(img)

        res = []
        for pos_y in range(self.height):
            res.append([])
            for pos_x in range(self.width):
                res[pos_y].append(self.smooth_adjacents_cells(pos_y, pos_x))
        return res
