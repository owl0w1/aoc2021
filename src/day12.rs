struct Graph {
    adj_list: Vec<Vec<usize>>,
    node_cap: Vec<bool>,
}
impl Graph {
    fn new(s: &str) -> Self {
        let mut names = std::collections::HashMap::from([("end", 0), ("start", 1)]);
        let mut adj_list = vec![Vec::new(); 2];
        let mut node_cap = vec![false; 2];
        for edge in s.split_ascii_whitespace() {
            let (from, to) = edge.split_once('-').unwrap();
            let next_idx = names.len();
            let from_idx = match names.entry(from) {
                std::collections::hash_map::Entry::Occupied(slot) => *slot.get(),
                std::collections::hash_map::Entry::Vacant(slot) => {
                    adj_list.push(Vec::new());
                    node_cap.push(from.as_bytes()[0].is_ascii_uppercase());
                    *slot.insert(next_idx)
                }
            };
            let next_idx = names.len();
            let to_idx = match names.entry(to) {
                std::collections::hash_map::Entry::Occupied(slot) => *slot.get(),
                std::collections::hash_map::Entry::Vacant(slot) => {
                    adj_list.push(Vec::new());
                    node_cap.push(to.as_bytes()[0].is_ascii_uppercase());
                    *slot.insert(next_idx)
                }
            };
            if from_idx != 0 && to_idx != 1 {
                adj_list[from_idx].push(to_idx);
            }
            if to_idx != 0 && from_idx != 1 {
                adj_list[to_idx].push(from_idx);
            }
        }
        Self { adj_list, node_cap }
    }

    fn path_count(
        &self,
        curr: usize,
        visited_mask: u32,
        twice_allowed: bool,
        mem: &mut std::collections::HashMap<(usize, u32, bool), u32>,
    ) -> u32 {
        if curr == 0 {
            return 1;
        }
        if let Some(count) = mem.get(&(curr, visited_mask, twice_allowed)) {
            return *count;
        }
        let mut next_visited_mask = visited_mask;
        let mut next_twice_allowed = twice_allowed;
        if !self.node_cap[curr] {
            if visited_mask & (1 << curr) != 0 {
                if !twice_allowed {
                    return 0;
                }
                next_twice_allowed = false;
            }
            next_visited_mask |= 1 << curr;
        }
        let mut count = 0;
        for neighbor in &self.adj_list[curr] {
            count += self.path_count(*neighbor, next_visited_mask, next_twice_allowed, mem);
        }
        mem.insert((curr, visited_mask, twice_allowed), count);
        count
    }
}

pub fn part1(input: &str) -> u32 {
    let graph = Graph::new(input);
    graph.path_count(1, 0, false, &mut Default::default())
}

pub fn part2(input: &str) -> u32 {
    let graph = Graph::new(input);
    graph.path_count(1, 0, true, &mut Default::default())
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day12.txt").unwrap()),
        4167
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day12.txt").unwrap()),
        98441
    );
}
