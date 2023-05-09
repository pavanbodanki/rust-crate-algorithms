pub mod kosaraju {
    //Importng necessary libraries
    use std::{
        collections::VecDeque,
        io::{stdin, stdout, Write},
    };

    /// The Kosaraju's algorithm is used to find strongly connected components.
    /// Given a directed graph represented as an adjacency list (Vec[Vec[]]), returns a vector of strongly connected components.
    ///
    /// # Arguments
    ///
    /// * `adj_list` - A directed graph represented as an adjacency list. Each vector in the adjacency list represents the vertices that the corresponding vertex has an outgoing edge to.
    ///
    /// # Returns
    ///
    /// * `list_of_scc` - A list of strongly connected components which are internally in sorted order represented as Vec[Vec[]].
    ///
    /// # Example
    /// ```
    /// use kosaraju::kosaraju_algorithm;
    ///
    /// let adj_list = vec![
    ///     vec![1],       // Node 0 has edge to node 1
    ///     vec![2],       // Node 1 has edge to node 2
    ///     vec![0, 3],    // Node 2 has edges to nodes 0 and 3
    ///     vec![4],       // Node 3 has edge to node 4
    ///     vec![5],       // Node 4 has edge to node 5
    ///     vec![3],       // Node 5 has edge to node 3
    /// ];
    ///
    /// kosaraju_algorithm(&adj_list);
    ///
    pub fn kosaraju_algorithm(adj_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        // Creating a reversed graph
        let mut adj_list_reversed = vec![Vec::new(); adj_list.len()]; //Creating empty adjacency list for the reversed graph
        for (u, e) in adj_list.iter().enumerate() {
            //Iterating through each vertex
            for &v in e {
                //Iterating through each neighbor adjacent to u
                adj_list_reversed[v].push(u); //Adding each vertex to the neighboring list in reversed graph
            }
        }

        let mut visited = vec![false; adj_list.len()]; //Creating a vector to keep track of visited vertices and initializing it with false
        let mut order = VecDeque::new(); //Creating an empty deque to store the vertices order
        for u in 0..adj_list.len() {
            if !visited[u] {
                dfs_reversed(u, &adj_list_reversed, &mut visited, &mut order); //Calling dfs_reversed function for unvisited vertices
            }
        }

        fn dfs_reversed(
            u: usize,
            adj_list: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            order: &mut VecDeque<usize>,
        ) {
            // DFS function for the reversed graph
            visited[u] = true; //Marking the vertex as visited
            for &v in &adj_list[u] {
                //Iterating through adjacent vertices of u
                if !visited[v] {
                    dfs_reversed(v, adj_list, visited, order); //Recursively calling the DFS function for unvisited
                }
            }
            order.push_front(u); //Adding vertex u to the front of the order deque
        }

        //perform DFS on the graph obtained above
        let mut visited = vec![false; adj_list.len()]; //Resetting the visited vector
        let mut list_of_scc = Vec::new(); //To store strongly connected components in a new vector
        while let Some(u) = order.pop_front() {
            // Iterating through vertices in the order obtained from previous DFS
            if !visited[u] {
                //If the vertex is unvisited, it belongs to a new SCC
                let mut scc = Vec::new(); //To store nodes in the current SCC
                dfs(u, adj_list, &mut visited, &mut scc); //Call dfs function for unvisited vertices
                scc.sort(); // Sort the SCC
                list_of_scc.push(scc); //Adding nodes in the current SCC
            }
        }
        list_of_scc.sort(); //Sorting the SCC

        fn dfs(
            u: usize,
            adj_list: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            scc: &mut Vec<usize>,
        ) {
            //DFS function for the original graph
            visited[u] = true; //Marking the vertex as visited
            scc.push(u); //Adding the vertex to the SCC
            for &v in &adj_list[u] {
                //Iterating through neighbors of u
                if !visited[v] {
                    dfs(v, adj_list, visited, scc); //Recursively calling the DFS function for unvisited neighbor
                }
            }
        }

        list_of_scc //Returning the list of SCCs
    }

    /// Performs Kosaraju's algorithm on a given directed graph represented as an adjacency list.
    /// Prints a vector of vectors, where each inner vector contains the nodes of a strongly connected component in sorted order.
    ///
    /// # Input
    ///
    /// * `Number of vertices` - Number of vertices in the graph. When this is 0, the output will be an empty vector.
    /// * `Number of neighbors for each vertex`
    /// * `Next neighbor for each vertex`
    ///
    /// # Output
    ///
    /// Prints Strongly connected components(SCC) of the graph
    ///
    /// # Sample input
    /// ```
    /// Please Enter Number of Vertices : 6
    /// Please enter the number of neighbors for vertex 0 : 1
    /// Please enter the next neighbor for vertex 0 : 1
    /// Please enter the number of neighbors for vertex 1 : 1
    /// Please enter the next neighbor for vertex 1 : 2
    /// Please enter the number of neighbors for vertex 2 : 2
    /// Please enter the next neighbor for vertex 2 : 0
    /// Please enter the next neighbor for vertex 2 : 3
    /// Please enter the number of neighbors for vertex 3 : 1
    /// Please enter the next neighbor for vertex 3 : 4
    /// Please enter the number of neighbors for vertex 4 : 1
    /// Please enter the next neighbor for vertex 4 : 5
    /// Please enter the number of neighbors for vertex 5 : 1
    /// Please enter the next neighbor for vertex 5 : 3
    ///
    /// ```
    /// # Sample output
    /// ```
    /// The strongly connected components are:
    /// [[0, 1, 2], [3, 4, 5]]

    pub fn kosaraju() {
        println!("******Kosaraju Algorithm*******");
        println!("******************");
        let mut buffer = String::new(); // Create a variable named buffer to read user input

        // Read the number of vertices from the user
        print!("Please Enter Number of Vertices : ");
        let _ = stdout().flush(); // Flushing stdout inorder to ensure that prompt is displayed
        stdin()
            .read_line(&mut buffer)
            .expect("Please Enter Valid number for vertices.");
        let vertices: usize = buffer
            .trim()
            .parse()
            .expect("Invalid input for number of vertices");
        buffer.clear();

        // Read the adjacency list for each vertex
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); vertices]; // Creating an empty adjacency list
        for i in 0..vertices {
            print!("Please enter the number of neighbors for vertex {} : ", i);
            let _ = stdout().flush();
            stdin()
                .read_line(&mut buffer)
                .expect("Please Enter Valid number for neighbors.");
            let num_neighbors: usize = buffer
                .trim()
                .parse()
                .expect("Invalid input for number of neighbors");
            buffer.clear();

            //neighbors input for each vertex
            let mut neighbors: Vec<usize> = Vec::new(); // Creating an empty list of neighbors
            for _j in 0..num_neighbors {
                print!("Please enter the next neighbor for vertex {} : ", i);
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut buffer)
                    .expect("Please Enter Valid number for next neighbor.");
                let neighbor: usize = buffer.trim().parse().expect("Invalid input for neighbor");
                buffer.clear(); // Clearing the buffer for next input
                neighbors.push(neighbor); // Adding neighbor to list of neighbors
            }
            adj_list[i] = neighbors;
        }

        // Call the kosaraju function with the adjacency list
        let res = kosaraju_algorithm(&adj_list);
        println!("******************");
        println!("The strongly connected components are:");
        println!("{:?}", res); // Printing the result of kosaraju algorithm
    }
}

#[cfg(test)]
mod tests {
    use super::kosaraju::kosaraju_algorithm;

    #[test]
    fn test_kosaraju_algorithm1() {
        let adj_list = vec![
            vec![1],
            vec![2],
            vec![3, 4],
            vec![0],
            vec![5],
            vec![6],
            vec![4, 7],
            vec![],
        ];

        let expected_result = vec![vec![0, 1, 2, 3], vec![4, 5, 6], vec![7]];

        let result = kosaraju_algorithm(&adj_list);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_kosaraju_algorithm2() {
        let adj_list = vec![vec![2, 3], vec![0], vec![1], vec![4], vec![]];

        let expected_result = vec![vec![0, 1, 2], vec![3], vec![4]];

        let result = kosaraju_algorithm(&adj_list);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_kosaraju_algorithm3() {
        //test for disconnected graph
        let adj_list = vec![
            vec![1],
            vec![2],
            vec![0, 3],
            vec![4],
            vec![3],
            vec![],
            vec![7],
            vec![5, 6],
        ];

        let expected_result = vec![vec![0, 1, 2], vec![3, 4], vec![5], vec![6, 7]];

        let result = kosaraju_algorithm(&adj_list);

        assert_eq!(result, expected_result);
    }
}
