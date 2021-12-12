use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(PartialEq)]
enum CaveSize {
    Small,
    Large,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum VertexIdx {
    Start,
    Index(usize),
    End,
}

struct Vertex {
    name: String,
    cave_size: CaveSize,
    neighbors: Vec<VertexIdx>,
}

impl Vertex {
    fn new(name: String, neighbors: Vec<VertexIdx>) -> Self {
        let cave_size = if name.chars().next().unwrap().is_uppercase() {
            CaveSize::Large
        } else {
            CaveSize::Small
        };

        Self {
            name,
            cave_size,
            neighbors,
        }
    }
}

struct Graph {
    start: Vertex,
    end: Vertex,
    verticies: Vec<Vertex>,
}

impl Graph {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut adjacency_list: HashMap<String, HashSet<String>> = HashMap::new();

        let lines = lines.map(|l| {
            let mut split = l.split('-');

            (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            )
        });

        // Parse the lines into the HashMap
        for (a, b) in lines {
            // This is an undirected graph, so we insert both ways
            if let Some(a_neighbors) = adjacency_list.get_mut(&a) {
                a_neighbors.insert(b.clone());
            } else {
                adjacency_list.insert(a.clone(), HashSet::from([b.clone()]));
            }

            if let Some(b_neighbors) = adjacency_list.get_mut(&b) {
                b_neighbors.insert(a);
            } else {
                adjacency_list.insert(b, HashSet::from([a]));
            }
        }

        // Convert the HashMap into a Graph

        let mut vertex_names = Vec::with_capacity(adjacency_list.len());

        for name in adjacency_list.keys() {
            if name != "start" && name != "end" {
                vertex_names.push(name.clone());
            }
        }

        let mut start = None;
        let mut end = None;
        let mut verticies = Vec::with_capacity(adjacency_list.len());

        for (name, neighbor_names) in adjacency_list {
            let neighbors = neighbor_names
                .iter()
                .map(|neighbor| match neighbor.as_str() {
                    "start" => VertexIdx::Start,
                    "end" => VertexIdx::End,
                    _ => VertexIdx::Index(vertex_names.iter().position(|n| n == neighbor).unwrap()),
                })
                .collect();

            let vertex = Vertex::new(name.clone(), neighbors);

            match name.as_str() {
                "start" => start = Some(vertex),
                "end" => end = Some(vertex),
                _ => verticies.push(vertex),
            }
        }

        Self {
            start: start.unwrap(),
            end: end.unwrap(),
            verticies,
        }
    }

    fn get_vertex(&self, idx: VertexIdx) -> &Vertex {
        match idx {
            VertexIdx::Start => &self.start,
            VertexIdx::End => &self.end,
            VertexIdx::Index(i) => &self.verticies[i],
        }
    }

    fn traverse_task1(&self, current_idx: VertexIdx, mut visited: HashSet<VertexIdx>) -> u32 {
        if current_idx == VertexIdx::End {
            return 1;
        }

        let vertex = self.get_vertex(current_idx);

        if vertex.cave_size == CaveSize::Small {
            visited.insert(current_idx);
        }

        let mut num_paths = 0;

        for &neighbor in vertex.neighbors.iter() {
            if !visited.contains(&neighbor) {
                num_paths += self.traverse_task1(neighbor, visited.clone());
            }
        }

        num_paths
    }

    fn traverse_task2(
        &self,
        current_idx: VertexIdx,
        mut visited: HashSet<VertexIdx>,
        twice: Option<VertexIdx>,
    ) -> u32 {
        if current_idx == VertexIdx::End {
            return 1;
        }

        let vertex = self.get_vertex(current_idx);

        if vertex.cave_size == CaveSize::Small {
            visited.insert(current_idx);
        }

        let mut num_paths = 0;

        for &neighbor in vertex.neighbors.iter() {
            if !visited.contains(&neighbor) {
                num_paths += self.traverse_task2(neighbor, visited.clone(), twice);
            }

            if visited.contains(&neighbor) && twice.is_none() && neighbor != VertexIdx::Start {
                num_paths += self.traverse_task2(neighbor, visited.clone(), Some(neighbor));
            }
        }

        num_paths
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_vertex = |v: &Vertex| -> std::fmt::Result {
            let neighbors_str = v
                .neighbors
                .iter()
                .map(|&i| self.get_vertex(i).name.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(f, "{} - {}", v.name, neighbors_str)?;

            Ok(())
        };

        print_vertex(&self.start)?;

        for v in self.verticies.iter() {
            print_vertex(v)?;
        }

        print_vertex(&self.end)?;

        Ok(())
    }
}

fn main() {
    let graph = Graph::parse(include_str!("input.txt").lines());

    // println!("{:?}", graph);
    println!("{}", graph.traverse_task1(VertexIdx::Start, HashSet::new()));
    println!(
        "{}",
        graph.traverse_task2(VertexIdx::Start, HashSet::new(), None)
    );
}
