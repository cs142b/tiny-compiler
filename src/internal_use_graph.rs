use std::collections::HashMap;

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


#[cfg(test)]
mod b_graph_tests {
    use super::BetterGraph;

    #[test]
    pub fn creation_test() {
        let new_graph_directed: BetterGraph<isize, isize> = BetterGraph::new_directed();
        let new_graph_undirected: BetterGraph<isize, isize> = BetterGraph::new_undirected();
        assert_eq!(new_graph_directed.node_count(), 0);
        assert_eq!(new_graph_undirected.node_count(), 0);
    }

    #[test]
    pub fn node_addition() {

        let mut new_graph_directed: BetterGraph<isize, isize> = BetterGraph::new_directed();
        let mut new_graph_undirected: BetterGraph<isize, isize> = BetterGraph::new_undirected();
        assert_eq!(new_graph_directed.node_count(), 0);
        new_graph_directed.add_node(10);
        assert_eq!(new_graph_directed.node_count(), 1);

        assert_eq!(new_graph_undirected.node_count(), 0);
        new_graph_undirected.add_node(10);
        assert_eq!(new_graph_undirected.node_count(), 1);
        assert_eq!(new_graph_directed.get_nodes()[0], 10);

    }



    #[test]
    pub fn edge_addition() {

        let mut new_graph_directed: BetterGraph<isize, isize> = BetterGraph::new_directed();
        let mut new_graph_undirected: BetterGraph<isize, isize> = BetterGraph::new_undirected();
        assert_eq!(new_graph_directed.node_count(), 0);
        new_graph_directed.add_node(10);


        assert_eq!(new_graph_directed.node_count(), 1);
        new_graph_directed.add_node(15);

        new_graph_directed.add_directed_edge(10, 15, 5);

        assert_eq!(new_graph_directed.edge_count(), 1);

        assert_eq!(new_graph_undirected.node_count(), 0);
        new_graph_undirected.add_node(10);
        assert_eq!(new_graph_undirected.node_count(), 1);
        assert_eq!(new_graph_directed.get_nodes()[0], 10);

    }

}




