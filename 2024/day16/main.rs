use itertools::{iproduct, Itertools};
use num::complex::Complex;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug)]
struct GridRC {
    values: Vec<Vec<char>>,
    nrows: i32,
    ncols: i32,
}

impl GridRC {
    fn get(&self, pos: &Complex<i32>) -> Option<char> {
        self.values
            .get(pos.re as usize)?
            .get(pos.im as usize)
            .copied()
    }

    fn set(&mut self, pos: &Complex<i32>, c: char) {
        self.values[pos.re as usize][pos.im as usize] = c;
    }

    fn print(&self) {
        for row in &self.values {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn find_char(&self, ch: char) -> Complex<i32> {
        for r in 0..self.nrows {
            for c in 0..self.ncols {
                if self.values[r as usize][c as usize] == ch {
                    return Complex::new(r, c);
                }
            }
        }
        Complex::new(-1, -1)
    }
}

fn parse(input: &str) -> (GridRC, Complex<i32>, Complex<i32>) {
    let grid_lines = input.lines().collect_vec();
    let nrows = grid_lines.len() as i32;
    let ncols = grid_lines[0].len() as i32;
    let grid = GridRC {
        values: grid_lines
            .iter()
            .map(|l| l.chars().collect_vec())
            .collect_vec(),
        nrows,
        ncols,
    };
    let start = grid.find_char('S');
    let end = grid.find_char('E');

    (grid, start, end)
}

fn setup_graph(
    grid: &GridRC,
) -> (
    Graph<(Complex<i32>, char), u32, Undirected>,
    HashMap<(Complex<i32>, char), NodeIndex>,
) {
    let mut graph = Graph::new_undirected();

    // Create horizontal and vertical nodes
    let mut nodes: HashMap<(Complex<i32>, char), NodeIndex> = HashMap::default();
    for (r, c) in iproduct!(0..grid.nrows, 0..grid.ncols) {
        let pos = Complex::new(r, c);
        if grid.get(&pos) == Some('#') {
            continue;
        }
        let hnode = graph.add_node((pos, 'h'));
        let vnode = graph.add_node((pos, 'v'));
        nodes.insert((pos, 'h'), hnode);
        nodes.insert((pos, 'v'), vnode);
        // Add "rotation" edge
        if grid.get(&pos) == Some('E') {
            graph.add_edge(hnode, vnode, 0);
        } else {
            graph.add_edge(hnode, vnode, 1000);
        }
    }

    // Add edges between neighbors
    for (r, c) in iproduct!(0..grid.nrows, 0..grid.ncols) {
        let pos = Complex::new(r, c);
        if grid.get(&pos) == Some('#') {
            continue;
        }

        let pos_right = pos + Complex::new(0, 1);
        if grid.get(&pos_right) != Some('#') {
            graph.add_edge(nodes[&(pos, 'h')], nodes[&(pos_right, 'h')], 1);
        }
        let pos_down = pos + Complex::new(1, 0);
        if grid.get(&pos_down) != Some('#') {
            graph.add_edge(nodes[&(pos, 'v')], nodes[&(pos_down, 'v')], 1);
        }
    }

    (graph, nodes)
}

fn find_best_path(
    graph: &Graph<(Complex<i32>, char), u32, Undirected>,
    nodes: &HashMap<(Complex<i32>, char), NodeIndex>,
    start: Complex<i32>,
    end: Complex<i32>,
) -> Option<(u32, Vec<NodeIndex>)> {
    let goal_nodes = [nodes[&(end, 'h')], nodes[&(end, 'v')]];
    astar(
        &graph,
        nodes[&(start, 'h')],
        |n| goal_nodes.contains(&n),
        |e| *e.weight(),
        |_| 0,
    )
}

fn part1(input: &str) -> u32 {
    let (grid, start, end) = parse(input);
    let (graph, nodes) = setup_graph(&grid);

    find_best_path(&graph, &nodes, start, end).unwrap().0
}

fn part2(input: &str) -> u32 {
    let (grid, start, end) = parse(input);
    let (graph, nodes) = setup_graph(&grid);

    let best_path = find_best_path(&graph, &nodes, start, end);
    let (min_path_len, mut best_nodes) = best_path.unwrap();
    let mut i = 0;

    // Save off nodes on the best path into a mutable vector
    // For each node, temporarily remove the node and see if the shortest path length is still the same
    // If it is, add the new nodes to the end of the vector to search through
    while i < best_nodes.len() {
        let mut test_graph = graph.clone();
        test_graph.remove_node(best_nodes[i]);
        let test_path = find_best_path(&test_graph, &nodes, start, end);
        match test_path {
            None => (),
            Some((len, test_path_nodes)) => {
                if len == min_path_len {
                    let mut new_best_nodes = best_nodes.clone();
                    test_path_nodes
                        .iter()
                        .filter(|n| !&best_nodes.contains(n))
                        .for_each(|n| new_best_nodes.push(*n));
                    best_nodes = new_best_nodes;
                }
            }
        }
        i += 1;
    }
    // Remove overlapping nodes (i.e. horizontal and vertical at same position)
    best_nodes
        .iter()
        .map(|n| graph.node_weight(*n).unwrap().0)
        .unique()
        .count() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
