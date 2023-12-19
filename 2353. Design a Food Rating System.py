from sortedcontainers import SortedList

class FoodRatings:
    def __init__(self, foods: List[str], cuisines: List[str], ratings: List[int]):
        self.food_info = {}
        self.by_cuisine = collections.defaultdict(lambda: SortedList(key=lambda x: (-x[1], x[0])))

        for food, cuisine, rating in zip(foods, cuisines, ratings):
            self.food_info[food] = (cuisine, rating)
            self.by_cuisine[cuisine].add((food, rating))

    def changeRating(self, food: str, newRating: int):
        cuisine, oldRating = self.food_info[food]
        self.by_cuisine[cuisine].remove((food, oldRating))
        self.by_cuisine[cuisine].add((food, newRating))
        self.food_info[food] = (cuisine, newRating)

    def highestRated(self, cuisine: str) -> str:
        return self.by_cuisine[cuisine][0][0]
# Your FoodRatings object will be instantiated and called as such:
# obj = FoodRatings(foods, cuisines, ratings)
# obj.changeRating(food,newRating)
# param_2 = obj.highestRated(cuisine)