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
            if !names.contains_key(from) {
                names.insert(from, names.len());
                adj_list.push(Vec::new());
                node_cap.push(from.as_bytes()[0].is_ascii_uppercase());
            }
            let from_idx = *names.get(from).unwrap();
            if !names.contains_key(to) {
                names.insert(to, names.len());
                adj_list.push(Vec::new());
                node_cap.push(to.as_bytes()[0].is_ascii_uppercase());
            }
            let to_idx = *names.get(to).unwrap();
            if from_idx != 0 && to_idx != 1 {
                adj_list[from_idx].push(to_idx);
            }
            if to_idx != 0 && from_idx != 1 {
                adj_list[to_idx].push(from_idx);
            }
        }
        Self { adj_list, node_cap }
    }

    fn path_count<const CYCLE: bool>(&self, curr: usize, visited: &mut Vec<bool>) -> u32 {
        if curr == 0 {
            return 1;
        }
        let mut count = 0;
        for &neighbor in &self.adj_list[curr] {
            match (visited[neighbor], self.node_cap[neighbor]) {
                (false, _) | (_, true) => {
                    visited[neighbor] = true;
                    count += self.path_count::<CYCLE>(neighbor, visited);
                    visited[neighbor] = false;
                }
                (true, false) => {
                    if CYCLE {
                        count += self.path_count::<false>(neighbor, visited);
                    }
                }
            }
        }
        count
    }
}

pub fn part1(input: &str) -> u32 {
    let graph = Graph::new(input);
    graph.path_count::<false>(1, &mut vec![false; graph.adj_list.len()])
}

pub fn part2(input: &str) -> u32 {
    let graph = Graph::new(input);
    graph.path_count::<true>(1, &mut vec![false; graph.adj_list.len()])
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
