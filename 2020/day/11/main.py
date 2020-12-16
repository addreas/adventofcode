import sys
lines = [l.strip() for l in sys.stdin]

neighbors = [(-1, -1), ( 0, -1), ( 1, -1),
             (-1,  0),           ( 1,  0),
             (-1,  1), ( 0,  1), ( 1,  1)]

def valid_neighbors(x, y, world):
    return [(dx, dy) for (dx, dy) in neighbors if 0 <= x + dx < len(world[y]) and 0 <= y + dy < len(world)]

def count_neighbors(x, y, world):
    return sum(world[y + dy][x + dx] == '#' for dx, dy in valid_neighbors(x, y, world))

def count_directions(x, y, world):
    count = 0
    for dx, dy in valid_neighbors(x, y, world):
        tx, ty = x + dx, y + dy
        while 0 <= tx < len(world[0]) and 0 <= ty < len(world):
            cell = world[ty][tx]
            if cell == '.':
                tx, ty = tx + dx, ty + dy
                continue

            if cell == '#':
                count += 1

            break

    return count

def generation(world, searcher, limit):
    world_next = []
    changed = 0
    for y, line in enumerate(world):
        line_next = ""
        for x, cell in enumerate(line):
            count = searcher(x, y, world)
            cell_next = cell
            if cell == 'L' and count == 0:
                cell_next = '#'
                changed += 1
            elif cell == '#' and count >= limit:
                cell_next = 'L'
                changed += 1
            line_next += cell_next
        world_next.append(line_next)
    return changed, world_next

def count_occupied(world):
    return sum(sum(cell == '#' for cell in line) for line in world)

def one():
    world = lines
    changed = True
    while changed:
        changed, world = generation(world, count_neighbors, 4)
    print(count_occupied(world))

def two():
    world = lines
    changed = True
    while changed:
        changed, world = generation(world, count_directions, 5)
    print(count_occupied(world))

one()
two()
