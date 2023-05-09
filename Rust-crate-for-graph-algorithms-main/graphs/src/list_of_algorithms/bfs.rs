pub mod bfs {

    use std::usize;
    use std::{
        collections::{HashSet, VecDeque},
        io::{stdin, stdout, Write},
    };

    /// A graph data structure with adjacency list representation.
    pub struct Graph {
        /// The edges of the graph stored as adjacency lists.
        pub edges: Vec<Vec<usize>>,
        /// The total number of vertices in the graph.
        pub vertices: usize,
    }

    impl Graph {
        pub fn new(vertices: usize) -> Self {
            Graph {
                edges: vec![Vec::new(); vertices],
                vertices,
            }
        }

        /// Adds an edge between two vertices in the graph.
        ///
        /// # Arguments
        ///
        /// * `u` - The index of the first vertex.
        /// * `v` - The index of the second vertex.
        ///
        /// # Example
        ///
        /// ```
        /// use my_crate::Graph;
        ///
        /// let mut graph = Graph::new(5);
        ///
        /// graph.add_edge(0, 1);
        /// graph.add_edge(1, 2);
        /// graph.add_edge(2, 3);
        /// graph.add_edge(3, 4);
        ///
        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
            //for undirected graphs
            self.edges[v].push(u);
        }

        ///BFS algorithm
        /// Performs a Breadth-First Search on a given graph represented as an adjacency list and returns visited vertices in the order they were visited.
        ///
        /// # Arguments
        ///
        /// * adj_list - A graph represented as an adjacency list. Each vector in the adjacency list represents the vertices that the corresponding vertex has an outgoing edge to.
        /// * start - The index of the vertex to start the Breadth-First Search from.
        ///
        /// # Returns
        ///
        /// * visited - A vector of visited vertices in the order they were visited during the Breadth-First Search.
        ///
        /// # Example
        ///```
        ///  use breadth_first_search::bfs;
        ///
        /// let adj_list = vec![
        /// vec![1, 2], // Node 0 has edges to nodes 1 and 2
        /// vec![3, 4], // Node 1 has edges to nodes 3 and 4
        /// vec![5], // Node 2 has edge to node 5  
        /// vec![6], // Node 3 has edge to node 6
        /// vec![], // Node 4 has no outgoing edges
        /// vec![7, 8], // Node 5 has edges to nodes 7 and 8
        /// vec![], // Node 6 has no outgoing edges
        /// vec![9], // Node 7 has edge to node 9
        /// vec![], // Node 8 has no outgoing edges
        /// vec![], // Node 9 has no outgoing edges
        /// ];
        /// let start_vertex = 0;
        ///  
        /// let visited = bfs(&adj_list, start_vertex);
        ///
        /// assert_eq!(visited, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        ///

        pub fn b_fs(&self, start: usize) -> Vec<usize> {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut visited_vec = Vec::new();

            visited.insert(start);
            queue.push_back(start);

            while let Some(u) = queue.pop_front() {
                println!("Visited node: {}", u);
                visited_vec.push(u);
                for &v in &self.edges[u] {
                    if !visited.contains(&v) {
                        visited.insert(v);
                        queue.push_back(v);
                    }
                }
            }
            visited_vec
        }
    }

    /// Performs Breadth First Search algorithm on a given graph represented as an adjacency list.
    /// Prints a graph, where each inner vector contains the nodes of a strongly connected component in sorted order.
    ///
    /// # Input
    /// * `no_of_vertices` - Input the number of vertices in the graph
    /// * `no_of_edges` - Input the number of edges in the graph
    /// * `source` - The source vertex of an edge in the graph
    /// * `destination` - The destination of an edge in the graph
    /// * `Start_vertex` - The start vertex where BFS algorithm starts from
    ///
    /// # Output
    ///
    /// Prints the visit order of the vertices of the graph
    ///
    /// # Sample input
    /// ```
    /// Please Enter Number of Vertices : 5
    ///Please Enter Number of edges in the graph : 5
    ///Source : 0
    ///Destination : 1
    ///Source : 0
    ///Destination : 2
    ///Source : 0
    ///Destination : 3
    ///Source : 2
    ///Destination : 1
    ///Source : 2
    ///Destination : 4
    ///Enter Starting Vertex : 0
    /// ```
    ///  # Sample output
    /// ```
    /// Visited node: 0
    /// Visited node: 1
    /// Visited node: 2
    /// Visited node: 3
    /// Visited node: 4
    /// ```
    pub fn bfs() {
        //read the number of vertices from the console
        let mut vertex = String::new();
        println!("*****BFS********");
        println!("****************************************************");
        //get the number of vertices
        print!("Please Enter Number of Vertices : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut vertex)
            .expect("Enter valid number of vertices");
        let vertices: usize = vertex.trim().parse().expect("Invalid input");
        //get number of edges in the graph
        let mut n_edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _ = stdout().flush();
        stdin().read_line(&mut n_edges).expect("Enter Valid Input");
        let n_edges: i32 = n_edges.trim().parse().expect("Invalid input for source");
        //assign the vertices to each edge from the console
        let g = add_edges(vertices, n_edges);
        //get the source vertex
        let mut start = String::new();
        print!("Enter Starting Vertex : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut start)
            .expect("Enter valid starting vertex ");
        let start: usize = start
            .trim()
            .parse()
            .expect("Invalid input for starting vertex");
        //call BFS implementation
        g.b_fs(start);
    }

    //to return the vertices of each edge as a graph
    pub fn add_edges(vertices: usize, edges: i32) -> Graph {
        //intialize a new graph with the required number of vertices
        let mut g = Graph::new(vertices);
        for _i in 0..(edges) {
            //intialize source and destination
            let mut s = String::new();
            let mut d = String::new();
            //get the source
            print!("Source : ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Please Enter Valid Input for .");
            let s: usize = s.trim().parse().expect("Invalid input for source");
            //get the destination
            print!("Destination : ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut d)
                .expect("Please Enter Valid Input for .");
            let d: usize = d.trim().parse().expect("Invalid input for source");
            //add edge with source and destination
            g.add_edge(s, d);
            //for undirected graphs
            g.add_edge(d, s);
        }
        //return graph in the form containing vertices of the
        return g;
    }
}
#[cfg(test)]
mod tests {
    use super::bfs::*;

    #[test]
    fn test_new_graph() {
        let g = Graph::new(5);
        assert_eq!(g.vertices, 5);
        assert_eq!(g.edges.len(), 5);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);
        //Run the BFS algorithm starting from vertex 2
        let visited = g.b_fs(0);

        //Check that the visited nodes match the expected set
        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(visited, expected);
        assert!(visited.contains(&0));
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
        assert!(visited.contains(&3));
        assert!(visited.contains(&4));
        assert_eq!(visited.len(), 5);
    }

    #[test]
    fn test_bfs() {
        //Create a new graph with 5 vertices
        let mut g = Graph::new(5);

        //Add edges to the graph
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        g.add_edge(2, 3);
        g.add_edge(3, 3);

        //Run the BFS algorithm starting from vertex 2
        let visited = g.b_fs(2);

        //Check that the visited nodes match the expected set
        let expected = vec![2, 0, 1, 3];
        assert_eq!(visited, expected);
        assert!(visited.contains(&0));
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
        assert!(visited.contains(&3));
        assert_eq!(visited.len(), 4);
    }
}
