/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (start, end, weight) = (edge.0.to_string(), edge.1.to_string(), edge.2); 
        // 解构edge，前两个参数转为String是为了之后的借用（防止转移所有权）
        // 以下出现的所有clone都是为了防止所有权转移

        if let Some(neighbor) = self.adjacency_table_mutable().get_mut(&start) {
            neighbor.push( (end.clone(), weight) );
        } else {
            self.adjacency_table_mutable()
                .insert( start.clone(), vec![ (end.to_string(), weight) ] );
        }

        if let Some(neighbor) = self.adjacency_table_mutable().get_mut(&end) {
            neighbor.push( (start.clone(), weight) );
        } else {
            self.adjacency_table_mutable()
                .insert( end.clone(), vec![ (start.to_string(), weight) ] );
        }

    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        if self.contains(node) {
            return false;
        }
        self.adjacency_table_mutable().insert( node.to_string(), vec![] );
        true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (start, end, weight) = edge;

        if !self.contains( start ) {
            self.add_node( start );
        }
        if !self.contains( end ) {
            self.add_node( end );
        }

        self.adjacency_table_mutable().get_mut(start).expect("Unable to get mut ref.")
            // 获得起点邻接表的可变引用，接着然后插入新的边
            // 为了令需要的邻接表可变，必须保证在所有获得引用的地方都用 mut
            .push( (end.to_string(), weight) );

        // 对终点边表进行同样的操作
        self.adjacency_table_mutable().get_mut( end ).expect("Unable to get mut ref.")
            .push( (start.to_string(), weight) );

    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}