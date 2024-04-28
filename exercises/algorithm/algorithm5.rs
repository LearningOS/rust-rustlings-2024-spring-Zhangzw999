/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO
        // BFS 广度优先搜索
        // 要求：先访问初始节点所有没有访问过的邻节点，
        // 再按上述顺序依次访问下一级邻节点，
        // 直到所有节点被访问过

        // 初始化
        // visited 用于标记图中的元素是否已经被访问过
        // visit_order 相当于栈，按访问顺序（广度优先）返回元素
        // queue 为双端队列，是BFS实现的关键
        let mut visit_order = vec![];
        let mut visited = vec![false; self.adj.len()];
        let mut queue = VecDeque::new();

        // 初始化开始的节点 start，并作为访问的第一个元素，
        // 同时令其进入队列
        visited[start] = true;
        queue.push_back(start);
        visit_order.push(start);

        // 接下来在 queue 前端获取一个节点作为当前操作的节点
        while let Some(current_node) = queue.pop_front() {
            // 在本题的结构 Graph 中，
            // 节点通过边连接的其他节点以邻接数组方式存储，
            // （由于邻接矩阵唯一，因此广度优先遍历序列也唯一）
            for &neighbor_node in &self.adj[current_node] {
                // 然后遍历当前节点连接的所有节点
                // 看这些节点有没有被访问过
                if !visited[neighbor_node] {
                    // 如果没有访问过，那就访问并修改访问标记
                    visit_order.push(neighbor_node);
                    queue.push_back(neighbor_node);
                    visited[neighbor_node] = true;
                }
            }
            // 直到所有节点全部被访问，则pop_front将返回None，从而退出循环
        }
        
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

