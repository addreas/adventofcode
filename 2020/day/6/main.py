with open("input.txt") as f: print(sum(len(set.intersection(*map(set, group))) for group in map(str.split, f.read().split("\n\n"))))
