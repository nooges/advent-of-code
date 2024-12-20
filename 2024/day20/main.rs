use itertools::{iproduct, Itertools};
use num::abs;
use num::complex::Complex;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use rustc_hash::FxHashMap as HashMap;

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

    fn find_chars(&self, ch: char) -> Vec<Complex<i32>> {
        iproduct!(0..self.nrows, 0..self.ncols)
            .filter(|&(r, c)| self.values[r as usize][c as usize] == ch)
            .map(|(r, c)| Complex::new(r as i32, c as i32))
            .collect()
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
    Graph<Complex<i32>, u32, Undirected>,
    HashMap<Complex<i32>, NodeIndex>,
) {
    let mut graph = Graph::new_undirected();
    let mut nodes: HashMap<Complex<i32>, NodeIndex> = HashMap::default();

    // Add nodes
    for (r, c) in iproduct!(0..grid.nrows, 0..grid.ncols) {
        let pos = Complex::new(r, c);
        if grid.get(&pos) == Some('#') {
            continue;
        }
        let node = graph.add_node(pos);
        nodes.insert(pos, node);
    }

    // Add edges between neighbors
    for (r, c) in iproduct!(0..grid.nrows, 0..grid.ncols) {
        let pos = Complex::new(r, c);
        if grid.get(&pos) == Some('#') {
            continue;
        }

        let pos_right = pos + Complex::new(0, 1);
        if grid.get(&pos_right) != Some('#') {
            graph.add_edge(nodes[&pos], nodes[&pos_right], 1);
        }
        let pos_down = pos + Complex::new(1, 0);
        if grid.get(&pos_down) != Some('#') {
            graph.add_edge(nodes[&pos], nodes[&pos_down], 1);
        }
    }

    (graph, nodes)
}

fn part1(input: &str) -> u32 {
    let (grid, start, end) = parse(input);
    let (graph, nodes) = setup_graph(&grid);

    // Run dijkstra from start to end
    let node_distances = dijkstra(&graph, nodes[&start], Some(nodes[&end]), |_| 1);
    dbg!(&node_distances);

    // Save off shortest path length
    let shortest_len = &node_distances.get(&nodes[&end]).unwrap();
    dbg!(shortest_len);

    // For each wall point, if U/D neighbor or L/R neighbor is not wall, calcuate difference of the node distances
    let walls = grid.find_chars('#');
    let mut num_good_cheats = 0;
    for wall in walls {
        let left = wall + Complex::new(0, -1);
        let right = wall + Complex::new(0, 1);
        let up = wall + Complex::new(-1, 0);
        let down = wall + Complex::new(1, 0);

        if let (Some(c1), Some(c2)) = (grid.get(&left), grid.get(&right)) {
            if c1 != '#' && c2 != '#' {
                let d1 = node_distances.get(&nodes[&left]).unwrap();
                let d2 = node_distances.get(&nodes[&right]).unwrap();
                println!("{wall}: {}", abs(d1 - d2) - 2);
                //println!("{}", abs(d1 - d2) - 2);
                if (abs(d1 - d2) - 2) >= 100 {
                    num_good_cheats += 1;
                }
            }
        }
        if let (Some(c1), Some(c2)) = (grid.get(&up), grid.get(&down)) {
            if c1 != '#' && c2 != '#' {
                let d1 = node_distances.get(&nodes[&up]).unwrap();
                let d2 = node_distances.get(&nodes[&down]).unwrap();
                println!("{wall}: {}", abs(d1 - d2) - 2);
                //println!("{}", abs(d1 - d2) - 2);
                if (abs(d1 - d2) - 2) >= 100 {
                    num_good_cheats += 1;
                }
            }
        }
    }

    num_good_cheats
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    //aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
