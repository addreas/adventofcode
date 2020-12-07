import os

// query := r"(\w+ \w+) bags contain ((\d+ \w+ \w+ bags?[.,])+|no other bags)"
// regex was broken :(
fn get_bags(rules []string) map[string]map[string]int {
	mut bags := map[string]map[string]int{}
	for _, line in rules {
		if line.contains('no other bags') {
			continue
		}
		parts := line.split(' ')
		mut inner_bags := map[string]int{}
		for i := 4; i < parts.len - 3; i += 4 {
			inner_bags['${parts[i+1]} ${parts[i+2]}'] = parts[i].int()
		}
		bags['${parts[0]} ${parts[1]}'] = inner_bags
	}
	return bags
}

fn get_reverse_bags(bags map[string]map[string]int) map[string][]string {
	mut reverse_bags := map[string][]string{}
	for k, v in bags {
		for bag, _ in v {
			reverse_bags[bag] << k
		}
	}
	return reverse_bags
}

fn bfs(root string, reverse_bags map[string][]string) int {
	mut queue := [root]
	mut visited := map[string]bool{}
	mut count := -1
	for queue.len > 0 {
		item := queue.pop()
		if !visited[item] {
			visited[item] = true
			count++
			for _, next in reverse_bags[item] {
				queue << next
			}
		}
	}
	return count
}

fn get_contents(bag string, bags map[string]map[string]int) int {
	mut count := 0
	for k, v in bags[bag] {
		count += v + v * get_contents(k, bags)
	}
	return count
}

fn main() {
	bags := get_bags(os.get_lines())
	reverse_bags := get_reverse_bags(bags)
	println(bfs('shiny gold', reverse_bags))
	println(get_contents('shiny gold', bags))
}
