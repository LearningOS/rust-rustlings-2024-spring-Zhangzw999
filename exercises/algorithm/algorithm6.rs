/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        //TODO
        // DFS 深度优先算法
        // 要求：从初始节点开始向远处访问，直到没有后继节点，
        // 然后回溯到最近的且连接未访问节点的节点
        // 重复上述过程

        // 本题限制只能用HashSet，已经访问的元素可以放进HashSet中
        // 以下因为应用递归，所以每一次调用该方法都会使得输入的 v 
        // 通过开头的两条语句设置为已经被访问，因此只需要往下套即可
        visit_order.push(v);
        visited.insert(v);

        for &neighbor_node in &self.adj[v] {
            if !visited.contains(&neighbor_node) {
                self.dfs_util(neighbor_node, visited, visit_order)
            }
        }

        // 以下为 DFS 提供了一种非递归的实现
        #[cfg(feature = "non-recursive")]
        {
            let mut stack = vec![];
            // 此时v即为上题的start
            stack.push(v);
            visit_order.push(v);
            visited.insert(v);
            while let Some(current_node) = stack.pop() {
                for &neighbor_node in &self.adj[current_node] {
                    if !visited.contains(&neighbor_node) {
                        stack.push(neighbor_node);
                        visit_order.push(neighbor_node);
                        visited.insert(neighbor_node);
                    }
                }
            }
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

