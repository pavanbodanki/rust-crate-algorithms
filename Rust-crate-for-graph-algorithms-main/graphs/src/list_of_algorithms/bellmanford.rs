pub mod bellmanford {
    //Importng necessary libraries
    use std::usize;
    use std::{
        cmp::Ordering,
        io::{stdin, stdout, Write},
    };
    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    struct Node {
        //Define new struct called Node which represent each Node of the graph
        vertex: usize,
        dist: i32,
    }
    pub struct Graph {
        ///representation using edge list
        edges: Vec<(usize, usize, i32)>,
        ///total no of vertices
        vertices: usize,
    }
    //Custom implementation of the Ord trait for the Node struct, which used to order nodes by distance
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            other.dist.cmp(&self.dist)
        }
    }

    impl Graph {
        pub fn new(vertices: usize) -> Self {
            //Constructor for new graph with the given number of vertices
            Graph {
                edges: Vec::new(),
                vertices,
            }
        }

        ///Adding edges to the graph
        pub fn add_edge(&mut self, u: usize, v: usize, w: i32) {
            self.edges.push((u, v, w));
        }
        ///Bellman-Ford algorithm
        /// Bellman ford algorithm is used to find the shortest node from one node to all other nodes in a weighted graph
        ///
        /// # Arguments
        /// * vector(s,d,w) - A vector representing the source, destination and weight
        /// * start - The index of the vertex to start the Bellman Ford algorithm from.
        ///
        /// # Returns
        ///
        /// * vector(s,d,w) -Returns the vector of the shortest distance from source to destination after Bellman Ford is run.
        ///
        /// # Example
        ///```
        ///
        /// let  = vec![
        /// vec![0,1,-1], // Node 0 has edges to node 1 and weight -1
        /// vec![0,2,4],  // Node 0 has edges to node 2 and weight 4
        /// vec![1,2,3],  // Node 0 has edges to node 2 and weight 4
        /// vec![3,2,5],  // Node 0 has edges to node 2 and weight 4
        /// vec![3,1,1],  // Node 0 has edges to node 2 and weight 4
        /// vec![1,3,2],  // Node 0 has edges to node 2 and weight 4
        /// vec![1,4,2],  // Node 0 has edges to node 2 and weight 4
        /// vec![4,3,-3], // Node 0 has edges to node 2 and weight 4
        /// ];
        /// let start_vertex = 0;
        ///  
        /// let visited = bfs(&adj_list, start_vertex);
        ///
        /// assert_eq!(visited, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        ///

        pub fn bellman_ford(&self, src: usize) -> Vec<i32> {
            //initialize all distances to max value
            let mut dist = vec![i32::max_value(); self.vertices];
            dist[src] = 0; //initialize distance from source vertex to the source as 0

            //loop for (vertices - 1) times
            for _ in 0..self.vertices + 1 {
                //For every edge (u, v) with weight w, relax the edge
                for (u, v, w) in &self.edges {
                    //relaxing the distances
                    if dist[*u] != i32::max_value() && dist[*u] + *w < dist[*v] {
                        dist[*v] = dist[*u] + *w;
                    }
                }
            }

            //check for negative cycles
            let mut _negative_cycle = false;
            for (u, v, w) in &self.edges {
                if dist[*u] != i32::max_value() && dist[*u] + *w < dist[*v] {
                    panic!("Negative weight cycle detected");
                }
            }

            //return the distances from source to every other vertex
            dist
        }
    }

    /// Performs Bellmanford algorithm on a given weighted graph.
    /// Prints a graph with shortest distance from one vertex to another vertex.
    ///
    /// # Input
    /// * `no_of_vertices` - Input the number of vertices in the graph
    /// * `no_of_edges` - Input the number of edges in the graph
    /// * `source` - The source vertex of an edge in the graph
    /// * `destination` - The destination of an edge in the graph
    /// * `weight` - The weight of the corresponding edge
    ///
    /// # Output
    ///
    /// Prints the shortest distance of the vertices in the graph
    ///
    /// # Sample input
    /// ```
    /// Please Enter Number of Vertices : 5
    ///Enter Source Vertex : 0
    ///Please Enter Number of edges in the graph : 8
    ///Source : 0
    ///Destination : 1
    ///Weight : -1
    ///Source : 0
    ///Destination : 2
    ///Weight : 4
    ///Source : 1
    ///Destination : 2
    ///Weight : 3
    ///Source : 3
    ///Destination : 2
    ///Weight : 5
    ///Source : 3
    ///Destination : 1
    ///Weight : 1
    ///Source : 1
    ///Destination : 3
    ///Weight : 2
    ///Source : 1
    ///Destination : 4
    ///Weight : 2
    ///Source : 4
    ///Destination : 3
    ///Weight : -3
    /// ```
    ///  # Sample output
    /// ```
    ///Distance from vertex 0 to vertex 0 is 0
    ///Distance from vertex 0 to vertex 1 is -1
    ///Distance from vertex 0 to vertex 2 is 2
    ///Distance from vertex 0 to vertex 3 is -2
    ///Distance from vertex 0 to vertex 4 is 1
    /// ```
    pub fn bellmanford() {
        //read the number of vertices and source from the console
        let mut vertex = String::new();
        let mut source = String::new();
        println!("********Bellman Ford***********");
        println!("****************************************************");
        //get the number of vertices
        print!("Please Enter Number of Vertices : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut vertex)
            .expect("Enter valid number of vertices");
        let vertices: usize = vertex.trim().parse().expect("Invalid input");
        //get the source vertex
        print!("Enter Source Vertex : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut source)
            .expect("Enter valid source vertex ");
        let source: usize = source.trim().parse().expect("Invalid input for source");
        //get number of edges in the graph
        let mut n_edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _ = stdout().flush();
        stdin().read_line(&mut n_edges).expect("Enter Valid Input");
        let n_edges: i32 = n_edges.trim().parse().unwrap_or_else(|_| {
            println!("Invalid input for number of edges, using default value of 0");
            0
        });

        //assign the weights to each edge from the console
        let e = add_weights(vertices, source, n_edges);
        //call bellman_ford implementation
        let dist = e.bellman_ford(source);
        //print the distances from the source vertex
        for (v, d) in dist.iter().enumerate() {
            println!("Distance from vertex {} to vertex {} is {}", source, v, d);
        }
    }

    //to return the weights of each branch as a graph containing source,destination and weight
    fn add_weights(vertices: usize, _source: usize, edges: i32) -> Graph {
        //intialize a new graph with the required number of vertices
        let mut g = Graph::new(vertices);
        for _i in 0..(edges) {
            //intialize source,destination and weights
            let mut s = String::new();
            let mut d = String::new();
            let mut w = String::new();
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
            //get the weight
            print!("Weight : ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut w)
                .expect("Please Enter Valid Input for .");
            let w: i32 = w.trim().parse().expect("Invalid input for source");
            //add edge with source,destination and weight
            g.add_edge(s, d, w);
        }
        //return graph in the form containing source,destination and weight of the edge
        return g;
    }
}
#[cfg(test)]
mod tests {
    use crate::list_of_algorithms::bellmanford::bellmanford::Graph;
    #[test]
    fn test_bellman_ford() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 5);
        g.add_edge(0, 2, 3);
        g.add_edge(1, 2, 2);
        g.add_edge(1, 3, 6);
        g.add_edge(2, 3, 7);
        g.add_edge(3, 4, 1);

        let dist = g.bellman_ford(0);
        assert_eq!(dist, vec![0, 5, 3, 10, 11]);
    }
    #[test]
    fn test_bellman_ford_edge() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, -1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 3);
        g.add_edge(1, 3, 2);
        g.add_edge(1, 4, 2);
        g.add_edge(3, 2, 5);
        g.add_edge(3, 1, 1);
        g.add_edge(4, 3, -3);

        let dist = g.bellman_ford(0);
        assert_eq!(dist, vec![0, -1, 2, -2, 1]);
    }

    #[test]
    #[should_panic(expected = "Negative weight cycle detected")]
    fn test_negative_cycle() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -5);
        g.add_edge(2, 0, 2);

        let _dist = g.bellman_ford(0);
    }
}
