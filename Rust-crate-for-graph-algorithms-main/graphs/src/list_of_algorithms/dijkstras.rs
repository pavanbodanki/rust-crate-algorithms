pub mod dijkstras {
    use std::{
        cmp::Ordering, // Importing Ordering to make node structure comparision based on distances
        collections::{HashSet}, // Importing BinaryHeap and HashSet to get priority Node and to mark visited Nodes
        io::{stdin, stdout, Write}, // Importing input/output library for reading user input and for printing output
    };

    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    #[doc(hidden)]
    struct Node {
        vertex: usize, // Vertex name in the Graph
        dist: i32,     // Distance from source to vertex
    }

    /// A graph data structure represented as an adjacency list.
    pub struct Graph {
        /// The adjacency list of the graph, where the i-th element contains a list of adjacent nodes for vertex i.
        adj_list: Vec<Vec<Node>>, // Adjacency list representation using Node structure
        /// The total number of vertices in the graph.
        vertices: usize, // Total number of vertices in the Graph
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            // Custom comparison function for Node to compare on dist
            other.dist.cmp(&self.dist) // Nodes will be compared by their distance from the source
        }
    }
    #[doc(hidden)]
    impl Graph {
        /// Constructs a new Graph with the specified number of vertices.
        ///
        /// # Arguments
        ///
        /// * `vertices`: The number of vertices in the graph.
        ///
        /// # Example
        ///
        /// ```
        /// use graph::Graph;
        ///
        /// let g = Graph::new(10);
        /// ```
        pub fn new(vertices: usize) -> Self {
            // Constructor for Graph structure
            Graph {
                adj_list: vec![Vec::new(); vertices], // Initializing adjacency list with empty vectors with size equal to vertices number
                vertices,
            }
        }

        /// Adds an undirected edge between two vertices in the graph, with the specified weight.
        ///
        /// # Arguments
        ///
        /// * `u`: The source vertex.
        /// * `v`: The destination vertex.
        /// * `w`: The weight of the edge.
        ///
        /// # Example
        ///
        /// ```
        /// use graph::{Graph, Node};
        ///
        /// let mut g = Graph::new(3);
        ///
        /// g.add_edge(0, 1, 10);
        /// g.add_edge(1, 2, 20);
        /// g.add_edge(2, 0, 30);
        ///
        /// ```
        pub fn add_edge(&mut self, u: usize, v: usize, w: i32) {
            // Function `add_edge` to add edges to the graphs
            self.adj_list[u].push(Node { vertex: v, dist: w }); // Adding Vertex `v` as adjacent vertex of Vertex `u`
            self.adj_list[v].push(Node { vertex: u, dist: w }); // Add Vertex `u` as adjacent vertex of Vertex `v`
        }

        /// performs Dijkstra's algorithm on a weighted graph to find the shortest path from a source vertex to every vertex in the graph.
        ///
        /// # Arguments
        ///
        /// * `self` - A reference to the graph object.
        /// * `src` - The index of the source vertex.
        ///
        /// # Returns
        ///
        /// A `Vec<i32>` containing the shortest distance from the source vertex to every other vertex in the graph.
        ///
        /// # Example
        ///
        /// ```
        /// use graph::Graph;
        ///
        /// let g = Graph::new(4);
        /// g.add_edge(0, 1, 1);
        /// g.add_edge(0, 2, 4);
        /// g.add_edge(1, 2, 2);
        /// g.add_edge(1, 3, 5);
        /// g.add_edge(2, 3, 1);
        ///
        /// let dist = g.dijkstra(0);
        /// assert_eq!(dist, vec![0, 1, 3, 4]);
        /// ```
        pub fn dijkstra(&self, src: usize) -> Vec<i32> {
            let mut dist = vec![i32::max_value(); self.vertices]; // Initializing all distances to max value So that we can select min distance and update the graph
            let mut visited_vertices = HashSet::new(); // To store the visited vertices
            dist[src] = 0; // Initializing distance from source to the source to 0
            let mut nodes = vec![Node {
                vertex: src,
                dist: dist[src],
            }]; // Creating a vector to store nodes for priority queue  

            // Loop till the nodes is empty
            while !nodes.is_empty() {   
                nodes.sort_by_key(|n| n.dist); // Sort the nodes vector by their distances
                let node = nodes.remove(0); // Get the node with minimum distance from the front of the vector
                let u = node.vertex;    
                // Checking if the vertex is already visited
                if visited_vertices.contains(&u) {
                    continue;
                } else {
                    visited_vertices.insert(u); // marking the vertex as visited by inserting into the HashSet
                }

                // For every adjacent vertex of u, relax the edge
                for Node { vertex: v, dist: w } in &self.adj_list[u] {
                    let new_dist = dist[u] + *w; // Calculating the new distance
                    if new_dist < dist[*v] {
                        // Check if the new distance is less than current distance
                        dist[*v] = new_dist; // Relax the distance
                        nodes.push(Node {
                            vertex: *v,
                            dist: dist[*v],
                        }); // Push the node into priority queue
                    }
                }
            }

            // Return the distances from source to every other vertex
            dist
        }
    }

    /// Performs Dijkstra's algorithm on a given directed graph represented as an adjacency list.
    /// Prints a vector of vectors, where each inner vector contains the nodes of dijkstras in sorted order.
    ///
    /// # Inputs
    ///
    /// * `vertices` - Total number of vertices in the graph.
    ///
    /// * `source` - A vertex in the graph from which min weight to be calculated to all other vertices in the graph.
    ///
    /// * `edges` - Total Number of edges in the graph.
    ///
    /// * `Source(s) Destination(d) Weight(w) ` - Source, Destination and Weight for each edge in the graph.  
    ///
    /// # Output
    ///
    /// Prints minimum weight from the source vertex to all vertices in the graph.
    ///
    /// # Sample Input
    /// ```
    /// Please Enter Number of Vertices in the Graph : 5
    /// Please Enter Source Vertex : 0
    /// Please Enter Number of edges in the graph : 3
    /// Please Enter Edge 1 values
    /// Source : 0
    /// Destination : 1
    /// Weight(>0) : 10
    /// Please Enter Edge 2 values
    /// Source : 0
    /// Destination : 2
    /// Weight(>0) : 5
    /// Please Enter Edge 3 values
    /// Source : 3
    /// Destination : 4
    /// Weight(>0) : 4
    ///
    /// ```
    /// # Sample Output
    /// ```
    /// Distance from vertex 0 to vertex 0 is 0
    /// Distance from vertex 0 to vertex 1 is 10
    /// Distance from vertex 0 to vertex 2 is 5
    /// Distance from vertex 0 to vertex 3 is 2147483647
    /// Distance from vertex 0 to vertex 4 is 2147483647
    pub fn dijkstras() {
        // Create two empty strings to store user input(Source & Vertices Count)
        let mut ve = String::new();
        let mut so = String::new();

        // Printing the introduction message
        println!("******Dijkstras Algorithm*******");
        println!("******************");

        // Prompting user to input number of vertices in the Graph
        print!("Please Enter Number of Vertices in the Graph : ");
        let _ = stdout().flush();

        // Reading user input for number of vertices, parsing it into an integer and handling errors
        stdin()
            .read_line(&mut ve)
            .expect("Please Enter Valid number for vertices.");
        let vertices: usize = ve
            .trim()
            .parse()
            .expect("Invalid input for number of vertices");

        // Prompting user to input the source vertex
        print!("Please Enter Source Vertex : ");
        let _ = stdout().flush();

        // Reading user input for source vertex and parsing it into an integer
        stdin()
            .read_line(&mut so)
            .expect("Please Enter Valid Input for Source.");
        let source: usize = so.trim().parse().expect("Invalid input for source");

        // Prompting user to input the number of edges
        let mut edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _ = stdout().flush(); // Flushing the output buffer to ensure prompt is displayed before taking input from user

        // Reading user input for number of edges and parsing it into an integer
        stdin()
            .read_line(&mut edges)
            .expect("Please Enter Valid Input for number of Edges.");
        let edges: i32 = edges
            .trim()
            .parse()
            .expect("Invalid input for number of edges");

        // Creating a new graph with the number of vertices entered by the user
        let mut g = Graph::new(vertices);

        // Initializing counter variable to keep track of the number of edges added
        let mut cnt = 0;

        // Loop Prompting user to input all edges source, destination and weight
        while cnt < edges {
            println!("Please Enter Edge {} values ", cnt + 1);
            let mut s = String::new();
            let mut d = String::new();
            let mut w = String::new();

            // Prompting user to input source vertex of current edge
            print!("Source : ");
            let _ = stdout().flush();

            // Reading user input for source vertex and parsing it into an integer
            stdin()
                .read_line(&mut s)
                .expect("Please Enter Valid Input for Source.");
            let s: usize = s.trim().parse().expect("Invalid input for source");

            // Prompting user to input destination vertex of current edge
            print!("Destination : ");
            let _ = stdout().flush();

            // Reading user input for destination vertex and parsing it into an integer
            stdin()
                .read_line(&mut d)
                .expect("Please Enter Valid Input for destination.");
            let d: usize = d.trim().parse().expect("Invalid input for destination");

            // Prompting user to input weight of current edge
            print!("Weight(>0) : ");
            let _ = stdout().flush();

            // Reading user input for edge weight and parsing it into an integer
            stdin()
                .read_line(&mut w)
                .expect("Please Enter Valid Input for weight.");
            let w: i32 = w.trim().parse().expect("Invalid input for weight");

            // Adding the current edge to the graph
            g.add_edge(s, d, w);

            // Increasing the edge counter
            cnt = cnt + 1;
        }

        // Calling Dijkstra's algorithm to find the shortest path from th e Source
        let dist = g.dijkstra(source);
        println!("******************");
        // Looping and printing the distances from Source to respective vertices
        for (v, d) in dist.iter().enumerate() {
            println!("Distance from vertex {} to vertex {} is {}", source, v, d);
        }
    }
}
// stdin, stdout and Write trait from std::io module

#[cfg(test)]
mod tests {

    use super::dijkstras::Graph;

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 2, 5);
        g.add_edge(1, 3, 1);
        g.add_edge(2, 1, 3);
        g.add_edge(2, 3, 8);
        g.add_edge(2, 4, 2);
        g.add_edge(3, 4, 4);
        let dist = g.dijkstra(0);
        assert_eq!(dist, vec![0, 8, 5, 9, 7]);
    }

    #[test]
    #[should_panic]
    fn test_empty_graph() {
        let g = Graph::new(0);
        g.dijkstra(0);
    }

    #[test]
    fn test_single_vertex() {
        let g = Graph::new(1);
        let dist = g.dijkstra(0);
        assert_eq!(dist, vec![0]);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 2, 5);
        g.add_edge(3, 4, 4);
        let dist = g.dijkstra(0);
        assert_eq!(dist, vec![0, 10, 5, i32::max_value(), i32::max_value()]);
    }

}
