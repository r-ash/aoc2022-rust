use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use itertools::iterate;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2022_rust::problem;

fn main() {
    let input = problem::load_raw(22);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
//     let solution_2 = solve2(&input);
//     println!("Puzzle 2 {}", solution_2);
}

#[derive(Debug)]
enum Instruction {
    RotateL,
    RotateR,
    Move(usize),
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Solid,
    Open,
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    no_cols: usize,
    no_rows: usize,
}

struct State {
    row: usize,
    col: usize,
    // direction R is 0, Down is 1, left is 2, up is 3
    direction: i8,
}

impl State {
    pub fn new(board: &Board) -> Self {
        let start_col = get_start_col(&board.tiles);
        State {
            row: 0,
            col: start_col,
            direction: 0,
        }
    }

    fn get_final_password(&self) -> usize {
        1000 * (self.row + 1) + 4 * (self.col + 1) + self.direction as usize
    }
}

fn move_one(&State { row, col, direction }: &State, board: &Board) -> State {
    match direction {
        0 => State {
            row,
            col: (col + 1) % board.no_cols,
            direction,
        },
        1 => State {
            row: (row + 1) % board.no_rows,
            col,
            direction,
        },
        2 => State {
            row,
            col: col.checked_sub(1).unwrap_or(board.no_cols - 1),
            direction,
        },
        3 => State {
            row: row.checked_sub(1).unwrap_or(board.no_rows - 1),
            col,
            direction,
        },
        _ => panic!("Unknown direction")
    }
}

fn run_instruction(state: State, instruction: Instruction, board: &Board) -> State {
    match instruction {
        Instruction::RotateR => {
            State {
                row: state.row,
                col: state.col,
                direction: (state.direction + 1).rem_euclid(4),
            }
        }
        Instruction::RotateL => {
            State {
                row: state.row,
                col: state.col,
                direction: (state.direction - 1).rem_euclid(4),
            }
        }
        Instruction::Move(steps) => {
            iterate(state, |state| move_one(state, board))
                .filter(|s| *board.tiles[s.row].get(s.col).unwrap_or(&Tile::Empty) != Tile::Empty)
                .take(steps + 1)
                .take_while(|s| board.tiles[s.row][s.col] == Tile::Open)
                .last()
                .unwrap()
        }
    }
}

fn get_start_col(tiles: &Vec<Vec<Tile>>) -> usize {
    tiles.first().unwrap().iter().position(|tile| matches!(tile, Tile::Open)).unwrap()
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+|R|L)").unwrap();
    }
    ;
    RE.find_iter(line)
        .map(|m| {
            let matched_str = m.as_str();
            if matched_str.starts_with("R") {
                Instruction::RotateR
            } else if matched_str.starts_with("L") {
                Instruction::RotateL
            } else {
                Instruction::Move(matched_str.parse().unwrap())
            }
        })
        .collect()
}

fn parse_inputs(input: &str) -> (Board, Vec<Instruction>) {
    let mut tiles = vec![];
    let mut instructions: Vec<Instruction> = vec![];
    let mut iter = input.lines().peekable();
    let mut no_cols = 0;
    let mut no_rows = 0;
    while let Some(line) = iter.next() {
        if iter.peek().is_none() {
            instructions = parse_instructions(line);
            break;
        }
        let mut row: Vec<Tile> = vec![];
        for c in line.chars() {
            if c == ' ' {
                row.push(Tile::Empty);
            } else if c == '#' {
                row.push(Tile::Solid);
            } else if c == '.' {
                row.push(Tile::Open);
            } else {
                panic!("Invalid char encountered {}", c);
            }
        }
        let no_chars = line.chars().count();
        if no_chars > no_cols {
            no_cols = no_chars;
        }
        if row.len() > 0 {
            tiles.push(row);
            no_rows += 1;
        }
    }
    (Board { tiles, no_cols, no_rows }, instructions)
}

fn solve1(input: &str) -> usize {
    let (board, instructions) = parse_inputs(input);
    let end_state = instructions.into_iter().fold(
        State::new(&board),
        |state, instruction| run_instruction(state, instruction, &board));
    return end_state.get_final_password();
}

// fn solve2(input: &str) -> u32 {
//     let mut lines = input.lines().peekable();
//     let mut sum: u32 = 0;
//     // Relying on that fact that if we can get 1 we can get 3 as this is a nice puzzle
//     while lines.peek().is_some() {
//         sum += get_priority(get_common_items(lines.next().unwrap(),
//                                              lines.next().unwrap(),
//                                              lines.next().unwrap()));
//     }
//     return sum;
// }


#[derive(Clone, Debug, Eq, Hash)]
struct Node {
    id: String,
    loc: (u32, u32),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn equivalent_node(a: Node, b: Node, edge_length: u32) -> bool {
    a.id != b.id &&
        (a.loc.0 == b.loc.0 && a.loc.1 == b.loc.1 + edge_length || // A above b
            a.loc.0 == b.loc.0 && a.loc.1 + edge_length == b.loc.1 || // B above A
            a.loc.0 == b.loc.0 + edge_length && a.loc.1 == b.loc.1 || // A left of B
            a.loc.0 + edge_length == b.loc.0 && a.loc.1 == b.loc.1)   // B left of A
}

#[derive(Clone, Debug, Eq)]
struct Edge {
    a: Node,
    b: Node,
}

#[derive(Clone, Debug, Eq)]
struct EquivalentNode {
    a: Node,
    b: Node,
}

impl<'a> PartialEq for EquivalentNode {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b || self.a == other.b && self.b == other.a
    }
}

impl<'a> Hash for EquivalentNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.a.id < self.b.id {
            self.a.id.hash(state);
            self.b.id.hash(state);
        } else {
            self.b.id.hash(state);
            self.a.id.hash(state);
        }
    }
}

fn identify_nodes<'a>(equivalent_nodes: &HashSet<EquivalentNode>, x: &'a Edge, y: &'a Edge) -> Option<(Node, EquivalentNode)> {
    let mut all_x_a = vec![x.a.clone()];
    let mut all_x_b = vec![x.b.clone()];
    for node in equivalent_nodes {
        if node.a == x.a {
            all_x_a.push(node.b.clone());
        } else if node.b == x.a {
            all_x_a.push(node.a.clone());
        }

        if node.a == x.b {
            all_x_b.push(node.b.clone());
        } else if node.b == x.b {
            all_x_b.push(node.a.clone());
        }
    }

    if all_x_a.contains(&y.a) {
        Some((x.a.clone(), EquivalentNode { a: x.b.clone(), b: y.b.clone() }))
    } else if all_x_a.contains(&y.b) {
        Some((x.a.clone(), EquivalentNode { a: x.b.clone(), b: y.a.clone() }))
    } else if all_x_b.contains(&y.a) {
        Some((x.b.clone(), EquivalentNode { a: x.a.clone(), b: y.b.clone() }))
    } else if all_x_b.contains(&y.b) {
        Some((x.b.clone(), EquivalentNode { a: x.a.clone(), b: y.a.clone() }))
    } else {
        None
    }
}

impl<'a> PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a.id == other.a.id && self.b.id == other.b.id ||
            self.a.id == other.b.id && self.b.id == other.a.id
    }
}

impl<'a> Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.a.id < self.b.id {
            self.a.id.hash(state);
            self.b.id.hash(state);
        } else {
            self.b.id.hash(state);
            self.a.id.hash(state);
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct EquivalentEdges<'a> {
    edge_1: &'a Edge,
    edge_2: &'a Edge,
}

impl<'a> Hash for EquivalentEdges<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.edge_1.a.id < self.edge_2.a.id || self.edge_1.b.id < self.edge_2.b.id {
            self.edge_1.hash(state);
            self.edge_2.hash(state);
        } else {
            self.edge_2.hash(state);
            self.edge_1.hash(state);
        }
    }
}

impl<'a> PartialEq for EquivalentEdges<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.edge_1 == other.edge_1 && self.edge_2 == other.edge_2 ||
            self.edge_1 == other.edge_2 && self.edge_2 == other.edge_1
    }
}

#[derive(Clone)]
struct Graph<'a> {
    graph: HashMap<&'a str, Vec<Edge>>,
    equivalent_edges: HashSet<EquivalentEdges<'a>>,
    equivalent_nodes: HashSet<EquivalentNode>,
}

impl<'a> Graph<'a> {
    fn get_unfolded_edges(&self) -> HashMap<&&str, &Vec<Edge>> {
        self.graph.iter().filter(|&(_, edges)| edges.len() < 4).collect()
    }

    // fn append_edges(&'a mut self, key: &str, edges: &'a Vec<&'a Edge>) {
    //     let current_edges = self.graph.get_mut(key).unwrap();
    //     for edge in edges {
    //         if !current_edges.contains(edge) {
    //             current_edges.push(edge);
    //         }
    //     }
    //     let edge = Edge { a: &n, b: &Node { id: String::from("c"), loc: (3, 2) } };
    //     current_edges.push(&edge);
    // }

    // fn append_duplicate(&mut self, edges: Vec<(&'a Edge, &'a Edge)>) {
    //     for (edge_a, edge_b) in edges {
    //         self.duplicate_edges.push((edge_a.clone(), edge_b.clone()))
    //     }
    // }


    fn fold_graph(&'a self) -> Graph<'a> {
        let mut equivalent_edges = self.equivalent_edges.clone();
        let mut equivalent_nodes = self.equivalent_nodes.clone();
        let mut graph = self.graph.clone();
        let unfolded = self.get_unfolded_edges();
        for (&key_x, &edges_x) in unfolded.iter() {
            for (&key_y, &edges_y) in unfolded.iter() {
                if *key_x != *key_y {
                    for edge_x in edges_x {
                        for edge_y in edges_y {
                            let node_x = Node { id: String::from(*key_x), loc: (0, 0) };
                            let node_y = Node { id: String::from(*key_y), loc: (0, 0) };
                            if edge_x != edge_y &&
                                !self.equivalent_edges.contains(&EquivalentEdges { edge_1: edge_x, edge_2: edge_y }) &&
                                !self.equivalent_nodes.contains(&EquivalentNode { a: node_x, b: node_y }) {
                                // println!("Testing edges {:?}, {:?},", edge_x, edge_y);
                                if let Some((common, equivalent)) = identify_nodes(&self.equivalent_nodes, edge_x, edge_y) {
                                    println!("common node, {}, has size {}", common.id, self.graph.get(common.id.as_str()).unwrap().len());
                                    if self.graph.get(common.id.as_str()).unwrap().len() == 4 {
                                        println!("Adding edges {:?}, {:?},", edge_x, edge_y);
                                        // In this case edge x and edge y must be the same edge
                                        // in the folded cube.
                                        // We can add them to our list of duplicates and
                                        // add their edges to each other
                                        equivalent_edges.insert(EquivalentEdges { edge_1: edge_x, edge_2: edge_y });
                                        equivalent_nodes.insert(equivalent.clone());
                                        graph = fold_edges(graph, equivalent.clone(), key_y, edges_x);
                                        graph = fold_edges(graph, equivalent, key_x, edges_y);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Graph { graph, equivalent_edges, equivalent_nodes }
    }
}

fn fold_edges<'a>(mut graph: HashMap<&'a str, Vec<Edge>>, equivalent_node: EquivalentNode, key: &str, edges: &'a Vec<Edge>) -> HashMap<&'a str, Vec<Edge>> {
    println!("folding edges for key {:?}, with eqiv nodes {:?} and edges {:?}", key, equivalent_node, edges);
    let current_edges = graph.get_mut(key).unwrap();
    let (fold_to, fold_from) = if equivalent_node.a.id == key {
        (equivalent_node.a.clone(), equivalent_node.b.clone())
    } else {
        (equivalent_node.b.clone(), equivalent_node.a.clone())
    };


    for edge in edges {
        let new: Edge = if edge.a.id == fold_from.id {
            Edge { a: fold_to.clone(), b: edge.b.clone() }
        } else {
            Edge { a: fold_to.clone(), b: edge.a.clone() }
        };
        if !current_edges.contains(&new) {
            current_edges.push(new);
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = problem::load_raw_test(22);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 6032);
//         let solution_2 = solve2(&input);
//         assert_eq!(solution_2, 70);
    }

    #[test]
    fn can_solve_cube_connections() {
        // This is what standard cube net looks like where we have
        //       A_____B
        //       |     |
        //       |     |
        //  C____D     E____F
        //  |               |
        //  |               |
        //  G____H     I____J
        //       |     |
        //       |     |
        //       K     L
        //       |     |
        //       |     |
        //       M_____N

        let a = Node { id: String::from("a"), loc: (1, 0) };
        let b = Node { id: String::from("b"), loc: (2, 0) };
        let c = Node { id: String::from("c"), loc: (0, 1) };
        let d = Node { id: String::from("d"), loc: (1, 1) };
        let e = Node { id: String::from("e"), loc: (2, 1) };
        let f = Node { id: String::from("f"), loc: (3, 1) };
        let g = Node { id: String::from("g"), loc: (0, 2) };
        let h = Node { id: String::from("h"), loc: (1, 2) };
        let i = Node { id: String::from("i"), loc: (2, 2) };
        let j = Node { id: String::from("j"), loc: (3, 2) };
        let k = Node { id: String::from("k"), loc: (1, 3) };
        let l = Node { id: String::from("l"), loc: (2, 3) };
        let m = Node { id: String::from("m"), loc: (1, 4) };
        let n = Node { id: String::from("n"), loc: (2, 4) };

        let ab = Edge { a: a.clone(), b: b.clone() };
        let ad = Edge { a: a.clone(), b: d.clone() };
        let be = Edge { a: b.clone(), b: e.clone() };
        let cd = Edge { a: c.clone(), b: d.clone() };
        let de = Edge { a: d.clone(), b: e.clone() };
        let ef = Edge { a: e.clone(), b: f.clone() };
        let cg = Edge { a: c.clone(), b: g.clone() };
        let dh = Edge { a: d.clone(), b: h.clone() };
        let ei = Edge { a: e.clone(), b: i.clone() };
        let fj = Edge { a: f.clone(), b: j.clone() };
        let gh = Edge { a: g.clone(), b: h.clone() };
        let hi = Edge { a: h.clone(), b: i.clone() };
        let hk = Edge { a: h.clone(), b: k.clone() };
        let ij = Edge { a: i.clone(), b: j.clone() };
        let il = Edge { a: i.clone(), b: l.clone() };
        let km = Edge { a: k.clone(), b: m.clone() };
        let kl = Edge { a: k.clone(), b: l.clone() };
        let ln = Edge { a: l.clone(), b: n.clone() };
        let mn = Edge { a: m.clone(), b: n.clone() };

        let full_graph = HashMap::from([
            ("a", vec![ab.clone(), ad.clone()]),
            ("b", vec![ab.clone(), be.clone()]),
            ("c", vec![cg.clone(), cd.clone()]),
            ("d", vec![ad.clone(), cd.clone(), de.clone(), dh.clone()]),
            ("e", vec![be.clone(), de.clone(), ef.clone(), ei.clone()]),
            ("f", vec![ef.clone(), fj.clone()]),
            ("g", vec![cg.clone(), gh.clone()]),
            ("h", vec![dh.clone(), gh.clone(), hi.clone(), hk.clone()]),
            ("i", vec![ei.clone(), hi.clone(), ij.clone(), il.clone()]),
            ("j", vec![fj.clone(), ij.clone()]),
            ("k", vec![hk.clone(), kl.clone(), km.clone()]),
            ("l", vec![il.clone(), kl.clone(), ln.clone()]),
            ("m", vec![km.clone(), mn.clone()]),
            ("n", vec![ln.clone(), mn.clone()]),
        ]);

        let graph = Graph {
            graph: full_graph,
            equivalent_edges: HashSet::new(),
            equivalent_nodes: HashSet::new(),
        };

        let folded = graph.fold_graph();
        let expected = HashSet::from([
            EquivalentEdges { edge_1: &ad, edge_2: &cd },
            EquivalentEdges { edge_1: &be, edge_2: &ef },
            EquivalentEdges { edge_1: &gh, edge_2: &hk },
            EquivalentEdges { edge_1: &ij, edge_2: &il }]);
        // After first pass
        assert_eq!(folded.equivalent_edges.len(), expected.len());
        for set in expected.iter() {
            assert!(folded.equivalent_edges.contains(set))
        }

        let expected_nodes = HashSet::from([
            EquivalentNode { a: a.clone(), b: c.clone() },
            EquivalentNode { a: b.clone(), b: f.clone() },
            EquivalentNode { a: g.clone(), b: k.clone() },
            EquivalentNode { a: l.clone(), b: j.clone() }
        ]);
        assert_eq!(folded.equivalent_nodes.len(), expected_nodes.len());
        assert_eq!(folded.equivalent_edges.len(), expected.len());
        for node in expected_nodes.iter() {
            assert!(folded.equivalent_nodes.contains(node))
        }

        // 2nd pass
        println!("{:?}", folded.graph.get("a").unwrap());
        println!("{:?}", folded.graph.get("m").unwrap());
        let folded = folded.fold_graph();
        let expected = HashSet::from([
            EquivalentEdges { edge_1: &ad, edge_2: &cd },
            EquivalentEdges { edge_1: &be, edge_2: &ef },
            EquivalentEdges { edge_1: &gh, edge_2: &hk },
            EquivalentEdges { edge_1: &ij, edge_2: &il },
            EquivalentEdges { edge_1: &ab, edge_2: &mn },
            EquivalentEdges { edge_1: &km, edge_2: &cg },
            EquivalentEdges { edge_1: &ln, edge_2: &fj }]);

        println!("{:?}", folded.equivalent_nodes);
        println!("{:?}", folded.equivalent_edges);
        //assert_eq!(folded.equivalent_edges.len(), expected.len());
        for set in expected.iter() {
            assert!(folded.equivalent_edges.contains(set))
        }

        let expected_nodes = HashSet::from([
            EquivalentNode { a: a.clone(), b: c.clone() },
            EquivalentNode { a: b.clone(), b: f.clone() },
            EquivalentNode { a: g.clone(), b: k.clone() },
            EquivalentNode { a: l.clone(), b: j.clone() },
            EquivalentNode { a: a.clone(), b: m.clone() },
            EquivalentNode { a: b.clone(), b: n.clone() },
            EquivalentNode { a: c.clone(), b: m.clone() },
            EquivalentNode { a: f.clone(), b: n.clone() },
        ]);
        assert_eq!(folded.equivalent_nodes.len(), expected_nodes.len());
        assert_eq!(folded.equivalent_edges.len(), expected.len());
        for node in expected_nodes.iter() {
            assert!(folded.equivalent_nodes.contains(node))
        }

        //
        // graph.find_equivalent_edges();
        // // After 2nd pass
        // assert_eq!(graph.duplicate_edges,
        //            vec![(ad.clone(), cd.clone()), (be.clone(), ef.clone()), (gh.clone(), hk.clone()), (ij.clone(), il.clone()),
        //                 (ab.clone(), mn.clone()), (km.clone(), cg.clone()), (ln.clone(), fj.clone())]
        // );
    }
}