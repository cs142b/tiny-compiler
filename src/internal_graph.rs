// use std::{
//     collections::{HashMap, HashSet, LinkedList, VecDeque},
//     hash::Hash,
// };

// struct Node<T> {
//     data: T,
// }

// struct HyperEdge<T, E> {
//     edges: std::collections::HashSet<Node<T>>,
//     edge_weights: std::collections::HashSet<E>,
// }

// struct HyperEdgeGraph<T, E> {
//     edges: HashSet<HyperEdge<T, E>>,
// }
// type InternalCollection<T> = HashSet<T>;
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum GraphType {
//     Directed,
//     Undirected,
// }
// #[derive(Debug)]
// struct BasicUnGraph<T>
// where
//     T: std::hash::Hash + Eq + PartialEq,
// {
//     graph_structure: HashMap<T, InternalCollection<T>>,
// }

// impl<T> BasicUnGraph<T>
// where
//     T: Hash + Eq + PartialEq + Copy,
// {
//     fn new() -> Self {
//         let g_struct = HashMap::<T, InternalCollection<T>>::new();
//         Self {
//             graph_structure: g_struct,
//         }
//     }

//     fn add_edge(&mut self, t1: &T, t2: &T) {
//         // self.graph_structure.insert(T)
//         if self.graph_structure.contains_key(t1) {
//             let curr = self.graph_structure.get_mut(t1);
//             let curr = curr.unwrap();
//             curr.insert(*t2);
//         } else {
//             self.graph_structure
//                 .insert(*t1, InternalCollection::<T>::new());
//             let curr = self.graph_structure.get_mut(t1);
//             let curr = curr.unwrap();
//             curr.insert(*t2);
//         }

//         if self.graph_structure.contains_key(t2) {
//             let curr = self.graph_structure.get_mut(t2);
//             let curr = curr.unwrap();
//             curr.insert(*t1);
//         } else {
//             self.graph_structure
//                 .insert(*t2, InternalCollection::<T>::new());
//             let curr = self.graph_structure.get_mut(t2);
//             let curr = curr.unwrap();
//             curr.insert(*t1);
//         }
//     }

//     fn remove_edge(&mut self, t1: &T, t2: &T) {
//         if self.graph_structure.contains_key(t1) && self.graph_structure.contains_key(t2) {
//             self.graph_structure.get_mut(t1).unwrap().remove(t2);
//             self.graph_structure.get_mut(t2).unwrap().remove(t1);
//         }
//     }
//     fn insert_edge(&mut self, t1: &T, t2: &T) {
//         if self.graph_structure.contains_key(t1) && self.graph_structure.contains_key(t2) {
//             self.graph_structure.get_mut(t1).unwrap().insert(*t2);
//             self.graph_structure.get_mut(t2).unwrap().insert(*t1);
//         }
//     }
//     fn remove_node(&mut self, t1: &T) {
//         if self.graph_structure.contains_key(t1) {
//             let tmp = self.graph_structure.get(t1).unwrap().clone();

//             for connected_node in tmp {
//                 self.remove_edge(t1, &connected_node);
//             }

//             self.graph_structure.remove(t1);
//         }
//     }

//     fn get_neighbors(&self, t1: &T) -> Option<&InternalCollection<T>> {
//         self.graph_structure.get(t1)
//     }
// }

// // type GraphObject<N, E> = HashSet<(N, E)>;

// type GraphObject<N, E> = (N, E);
// type InternalGraphCollection<N, E> = HashSet<GraphObject<N, E>>;
// type BetterGraphSize = usize;
// #[derive(Debug, PartialEq, Eq)]
// pub enum EdgeDirection {
//     Incoming,
//     Outgoing
// }
// // pub enum Directed {}
// // pub enum Undirected {}
// #[derive(Clone)]
// struct BetterGraph<N: Copy, E: Copy>
// where
//     N: std::hash::Hash + Eq + PartialEq,
// {
//     graph_store: HashMap<N, InternalGraphCollection<N, E>>,
//     graph_type: GraphType,
// }

// impl<N: Copy, E: Copy> BetterGraph<N, E>
// where
//     N: std::hash::Hash + Eq + PartialEq,
//     Self: Copy,
//     E: std::hash::Hash + Eq + PartialEq,
// {
//     pub fn new(gtype: GraphType) -> Self {
//         let h: HashMap<N, InternalGraphCollection<N, E>> = HashMap::new();
//         Self {
//             graph_store: h,
//             graph_type: gtype,
//         }
//     }
//     pub fn new_undirected() -> Self {
//         BetterGraph::new(GraphType::Undirected)
//     }
//     pub fn new_directed() -> Self {
//         BetterGraph::new(GraphType::Directed)
//     }

//     pub fn add_edge_directed(&mut self, from: N, to: N, e: E) -> Result<(), ()>
//     where
//         N: std::hash::Hash + Eq + PartialEq,
//     {
//         if self.graph_type != GraphType::Directed {
//             return Err(());
//         }
//         self.add_directed_edge_subroutine(from, to, e)
//     }

//     pub fn add_edge_undirected(&mut self, n1: &N, n2: &N, edge_weight: &E) -> Result<(), ()>
//     {
//         if self.add_directed_edge_subroutine(*n1, *n2, *edge_weight) == Ok(())
//             && self.add_directed_edge_subroutine(*n1, *n2, *edge_weight) == Ok(())
//         {
//             return Ok(());
//         } else {
//             return Err(());
//         }
//     }

//     fn add_directed_edge_subroutine(&mut self, from: N, to: N, edge_weight: E) -> Result<(), ()> {
//         let from_node = self.graph_store.get_mut(&from);
//         if from_node != None {
//             let from_node = from_node.unwrap();
//             let graph_obj_to_add: GraphObject<N, E> = (to, edge_weight);
//             from_node.insert(graph_obj_to_add);
//             return Ok(());
//         }
//         Err(())
//     }
    
//     fn remove_edge_directed_subroutine (&mut self, from: N, to: N) -> Result<(), ()> {
//         // returns Err if one of the two edges DNE
//         if self.graph_store.get(&from) == None && self.graph_store.get(&from) == None {
//             return Err(())
//         } 

//         Ok(())

//     }

//     fn contains_edge_subroutine(&self, from: N, to: N) -> bool {
//         let curr = self.graph_store.get(&from); 
//         if curr == None {
//             return false
//         }
    
//         let curr = curr.unwrap(); 
//         for graph_obj in curr {
//             if graph_obj.0 == to {
//                 return true
//             }
//         }
//         false
//     }

//     fn get_edge_weight(&self, from: N, to: N) -> Option<E> {

//         let curr = self.graph_store.get(&from); 
//         if curr == None {
//             return None
//         }
    
//         let curr = curr.unwrap(); 
//         for graph_obj in curr {
//             if graph_obj.0 == to {
//                 return Some(graph_obj.1) 
//             }
//         }
//         None 
//     }

//     fn get_neighbors_undirected(&self, node: N) -> Vec<N> {
//         let mut v = Vec::<N>::new(); 
//         let node_set = self.graph_store.get(&node);

//         if node_set == None {
//             eprintln!("Attempted to access a variable that don't have shit and doesn't actually exist");
//             return v; 
//         } 

//         for (curr_node, _) in node_set.unwrap() {
//             v.push(*curr_node)
//         }
//         v
//     }

//     fn get_neighbors_directed(&self, node: N, edge_direction: EdgeDirection) -> Vec<N> {

//         if edge_direction == EdgeDirection::Outgoing {
//             return self.get_neighbors_undirected(node)
//         } 
//         let mut v = Vec::<N>::new(); 

//         for (n, _) in &self.graph_store {
//             if self.contains_edge_subroutine(*n, node) {
//                 v.push(*n);
//             }
//         }
//         v
//     }

//     fn add_node(&mut self, node_to_add: N) -> Result<(), ()> {
//         if self.graph_store.contains_key(&node_to_add) {
//             return Err(())
//         }
//         self.graph_store.insert(node_to_add, InternalGraphCollection::<N, E>::new());
//         Ok(())
//     }

//     fn contains_node(&self, n: &N) -> bool {
//         self.graph_store.contains_key(n)
//     }

//     fn remove_edge_undirected(&mut self, node1: N, node2: N) {
//         self.remove_edge_directed_subroutine(node1, node2); 

//         self.remove_edge_directed_subroutine(node2, node1); 
//     }
//     fn remove_node_undirected (&mut self, node_to_remove: N) -> Result<BetterGraphSize, ()> {
//         let neighbors = self.get_neighbors_undirected(node_to_remove);
//         for neighbor in neighbors {
//             self.remove_edge_undirected(neighbor, node_to_remove);
//         }

//         self.graph_store.remove(&node_to_remove);
//         Ok(self.graph_store.len())
//     }

//     pub fn get_edge_count (&self) -> usize {
//         let mut tmp: usize = 0; 
//         for (_, set) in &self.graph_store {
//             tmp += set.len();
//         }

//         if self.graph_type == GraphType::Directed {
//             return tmp
//         } else {
//             return tmp/2
//         }
//     }

//     pub fn get_node_count(&self) -> usize {
//         self.graph_store.len()
//     }


// }




// struct BestGraph ()




type InternalGraphObject<N, E> = (N, E);
type InternalGraphCollection<N> = Vec<N>;
type ResultNoMessage = Result<(), ()>;
#[derive(Debug, Eq, PartialEq)]
enum GraphType {
    Undirected,
    Directed,
}

#[derive(Debug, Eq, PartialEq)]
enum EdgeType {
    Incoming, 
    Outgoing
}


struct BetterGraph<N: Copy, E: Copy>
where
    N: std::hash::Hash + Eq + PartialEq,
{
    graph_store: HashMap<N, InternalGraphCollection<InternalGraphObject<N, E>>>,
    graph_type: GraphType,
}

impl<N: Copy, E: Copy> BetterGraph<N, E> where N: std::hash::Hash + Eq + PartialEq {
    pub fn new(gtype: GraphType) -> Self {

        let igs: HashMap<N, InternalGraphCollection<InternalGraphObject<N, E>>> = HashMap::new(); 
        Self { graph_store: igs, graph_type: gtype}
    }

    pub fn new_undirected() -> Self {
        // let igc = InternalGraphCollection::<InternalGraphObject<N, E>>::new();
        let igs: HashMap<N, InternalGraphCollection<InternalGraphObject<N, E>>> = HashMap::new(); 
        Self {
            graph_store: igs,
            graph_type: GraphType::Undirected
        }


    }


    pub fn new_directed() -> Self {
        // let igc = InternalGraphCollection::<InternalGraphObject<N, E>>::new();
        let igs: HashMap<N, InternalGraphCollection<InternalGraphObject<N, E>>> = HashMap::new(); 
        Self {
            graph_store: igs,
            graph_type: GraphType::Directed
        }
    }

    pub fn add_directed_edge(&mut self, from: N, to: N, weight: E) -> ResultNoMessage {
        if self.graph_store.contains_key(&from) && self.graph_store.contains_key(&to) {
            let curr = self.graph_store.get_mut(&from).unwrap();
            curr.push((to, weight));
            return Ok(()) 
        } 
        Err(())
    }

    pub fn add_undirected_edge(&mut self, from: N, to: N, weight: E) -> ResultNoMessage {
        let _ = self.add_directed_edge(from, to, weight);
        
        self.add_directed_edge(to, from, weight)
    }

    pub fn add_node(&mut self, node_to_add: N) -> ResultNoMessage {
        if self.graph_store.contains_key(&node_to_add) {
            return Err(())
        } 
        self.graph_store.insert(node_to_add, Vec::<(N, E)>::new());
        Ok(())
    }

    pub fn remove_node(&mut self, rem_node: N) -> ResultNoMessage {
        self.remove_node_directed(rem_node)
    }

    fn remove_edge_directed(&mut self, from: N, to: N) -> ResultNoMessage {
        if !self.edge_exists_directed(from, to) {
            return Err(())
        }
        let from_vec = self.graph_store.get_mut(&from).unwrap();
        let mut idx = 0; 
        for (node, _weight) in from_vec.iter() {
            if *node == to {
                break;
            } 
            idx += 1; 
        }

        from_vec.swap_remove(idx);

        
        Ok(())
    }

    fn remove_edge_undirected(&mut self, n1: N, n2: N) -> ResultNoMessage {
        if !self.edge_exists_undirected(n1, n2) {
            return Err(())
        }

        // should never fail past this point
        let _ = self.remove_edge_directed(n1, n2);
        self.remove_edge_directed(n2, n1)
    }
    fn remove_node_directed(&mut self, rem_node: N) -> ResultNoMessage {

        if !self.graph_store.contains_key(&rem_node) {
            return Err(())
        }
        let incoming_neighbors = self.get_neighbors_directed(rem_node, EdgeType::Incoming);
        // do not need to deal with the inverse case as the garbage collection is already done internally her eanyway
        // in the other case the node is removed so no need to deal with that shit

        for neighbor in incoming_neighbors {
            let _ = self.remove_edge_directed(neighbor, rem_node);
        }

        Ok(())
    }

    fn remove_node_undirected(&mut self, rem_node: N) -> ResultNoMessage {
        self.remove_node_directed(rem_node)
    }

    fn get_neighbors_undirected (&self, n1: N) -> Vec<N>{
        // self.graph_store.get(&n1)
        let mut ret_vec = Vec::<N>::new(); 
        let n1_vec = self.graph_store.get(&n1);
        match n1_vec {
            
            Some(_) => {
                for (node, _edge) in n1_vec.unwrap() {
                    ret_vec.push(*node);
                }

                ret_vec
            },
            None => {
                ret_vec
            }
        }

        
    }

    fn get_neighbors_directed(&self, n1: N, e_type: EdgeType) -> Vec<N> {
        if e_type == EdgeType::Outgoing {
            return self.get_neighbors_undirected(n1);
        } else {
            let mut ret_vec =Vec::<N>::new();  
            for (node, _vec_of_neighbors) in &self.graph_store {
                if self.edge_exists_directed(*node, n1) {
                    ret_vec.push(*node);
                }
            }
            ret_vec
        }
    }

    fn edge_exists_directed (&self, from: N, to: N) -> bool {
        if !self.graph_store.contains_key(&from) || !self.graph_store.contains_key(&to) {
            return false;
        }

        let from_vec = self.graph_store.get(&from).unwrap();
        // from_vec.contains(x)
        for (node, _e_weight) in from_vec {
            if *node == to {
                return true
            }
        }

        false

        
    }

    fn edge_exists_undirected(&self, n1: N, n2: N) -> bool {
        self.edge_exists_directed(n1, n2) && self.edge_exists_directed(n2, n1)
    }

    pub fn node_count (&self) -> usize {
        self.graph_store.len()
    }

    pub fn edge_count (&self) -> usize {
        let mut running_total = 0; 
        for v in self.graph_store.values() {
            running_total += v.len();
        }

        if self.graph_type == GraphType::Undirected {
            return running_total / 2
        } else {
            return running_total
        }
    }

    pub fn get_nodes (&self) -> Vec<N> {
        let mut v = Vec::<N>::new();
        
        for key in self.graph_store.keys() {
            v.push(*key);
        } 
        v
    }

    /// this reutrns all the directed edges in the graph
    /// this should be read as (NodeFrom, NodeTo, EdgeWeight)
    pub fn get_edges_directed(&self) -> Vec<(N, N, E)> {
        let mut v = Vec::<(N, N, E)>::new(); 
        for node in self.graph_store.keys() {
            let outgoing_neighbors = self.graph_store.get(node).unwrap();
            for neighbor in outgoing_neighbors {
                let tmp = (*node, neighbor.0, neighbor.1);

                v.push(tmp);
            }
        }

        v
    }

    // pub fn get_edges_directed (&self) -> Vec<(N, N)> {
    //     let nodes = self.get_nodes(); 

    //     for 
    // }

    pub fn is_directed(&self) -> bool {
        self.graph_type == GraphType::Directed
    }
    


}