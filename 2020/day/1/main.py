from itertools import combinations
from math import prod

with open('input.txt', 'r') as f:
    numbers = [int(i) for i in f.readlines()]

print(
   prod(next(x for x in combinations(numbers, 2) if sum(x) == 2020)),
   prod(next(x for x in combinations(numbers, 3) if sum(x) == 2020))
)
