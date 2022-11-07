use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use std::collections::{VecDeque, HashMap, HashSet};
use std::cmp::PartialEq;
use std::hash::Hash;
use std::fmt::Debug;

pub mod directed_graph {
    use super::*;

    #[derive(Clone)]
    pub struct Node<T> {
        value: T,
        children: Vec<Box<Node<T>>>,
    }
    
    impl<T> Node<T> 
    where 
        T: Clone + PartialEq,
    {
        pub fn new(value: T) -> Self {
            Node { value, children: vec![] }
        }
        pub fn join(&mut self, node: Self) -> Self {
            self.children.push(Box::new(node));
            self.clone()
        }
        pub fn depth_first_vec(&self) -> Vec<T> {
            let mut stack: Vec<&Node<T>> = vec![self];
            let mut list: Vec<T> = vec![];
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                list.push(current.value.clone());
                for node in current.children.iter() {
                    stack.push(node.as_ref());
                }
            }
            list
        }
        pub fn breadth_first_vec(&self) -> Vec<T> {
            let mut queue: VecDeque<&Node<T>> = VecDeque::from([self]);
            let mut list: Vec<T> = vec![];
            while !queue.is_empty() {
                let current = queue.pop_back().unwrap();
                list.push(current.value.clone());
                for node in current.children.iter() {
                    queue.push_front(node.as_ref());
                }
            }
            list
        }
        pub fn contains(&self, value: T) -> bool {
            let mut stack: Vec<&Node<T>> = vec![self];
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                if current.value == value { return true }
                for child in current.children.iter() {
                    stack.push(child.as_ref());
                }
            }
            false
        }
    }
}

pub mod graph {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Graph<Nid, N = (), E = ()> {
        nodes: HashMap<Nid, N>,
        adjacent: HashMap<Nid, Vec<(Nid, E)>>
    }

    impl<Nid, N, E> Graph<Nid, N, E> 
    where
        Nid: Hash + Eq
    {
        pub fn new() -> Graph<Nid, N, E> {
            Graph { nodes: HashMap::new(), adjacent: HashMap::new() }
        }

        pub fn insert_node(&mut self, node_id: Nid, node: N) -> Option<N>{
            self.nodes.insert(node_id, node)
        }
        
        pub fn add_edge(&mut self, from: Nid, to: Nid, edge: E) {
            let entry = self.adjacent.entry(from).or_default();
            entry.push((to, edge));
        }

        pub fn remove_node(&mut self, node_id: &Nid) -> Option<N> {
            self.nodes.remove(node_id)
        }

        pub fn remove_edges(&mut self, from: Nid, to: &Nid) -> Result<(), &'static str>{
            if self.adjacent.contains_key(&from) {
                let entry = self.adjacent.entry(from).or_default();
                entry.retain(|(id, _)| id != to); 
                Ok(())
            }else { Err("Node does not have any edges!") }            
        }

        pub fn get_node(&self, node_id: &Nid) -> Option<&N> {
            self.nodes.get(node_id)
        }

        pub fn get_edge(&self, from: &Nid, to: &Nid) -> Option<&E> {
            if let Some(vec) = self.adjacent.get(from) {
                vec.iter().find(|(node_id, _)| node_id == to).map(|(_, edge)| edge)
            }else { None }
        }

        pub fn get_adjacent(&self, node_id: &Nid) -> Vec<&Nid> {
            self.adjacent.get(node_id).unwrap().iter().map(|(node_id, _)| node_id).collect()
        }

        pub fn edges_from(&self, node_id: &Nid) -> Option<&Vec<(Nid, E)>> {
            self.adjacent.get(node_id)
        }

        pub fn edges_from_to(&self, from: &Nid, to: &Nid) -> Option<Vec<&E>> {
            if let Some(vec) = self.adjacent.get(from) {
                Some(vec.iter().filter(|(node, _)| node == to).map(|(_, edge)| edge).collect::<Vec<&E>>())
            }else { None }
        }

        pub fn edge_count(&self, from: &Nid, to: &Nid) -> usize {
            if let Some(vec) = self.adjacent.get(from) {
                vec.iter().filter(|(node, _)| node == to).count()
            }else { 0 }
        }

        pub fn iter_nodes(&self) -> impl Iterator<Item = (&Nid, &N)> {
            self.nodes.iter()
        }

        pub fn iter_edges(&self) -> impl Iterator<Item = (&Nid, &Vec<(Nid, E)>)> {
            self.adjacent.iter()
        }

        pub fn contains(&self, node_id: &Nid) -> bool {
            self.nodes.contains_key(node_id)
        } 
    }


    impl<Nid, N, E> Graph<Nid, N, E> 
    where
        Nid: Hash + Eq + Clone,
        E: PartialEq + Clone
    {
        pub fn push_undirected_edge(&mut self, from: Nid, to: Nid, edge: E) {
            self.add_edge(from.clone(), to.clone(), edge.clone());
            self.add_edge(to, from, edge);
        }

        pub fn flip_edge(&mut self, from: Nid, to: Nid, edge: E) -> Result<(), &'static str> {
            if self.remove_edge(from.clone(), &edge).is_ok() {
                self.add_edge(to, from, edge);
                Ok(())
            }else { Err("Could not flip the edge, invalid edge or node provided.") }
        }
    }

    impl<Nid, N, E> Graph<Nid, N, E> 
    where
        Nid: Hash + Eq,
        E: PartialEq + Clone
    {
        pub fn remove_edge(&mut self, from: Nid, edge: &E) -> Result<(), &'static str> {
            if self.adjacent.contains_key(&from) {
                let entry = self.adjacent.entry(from).or_default();
                entry.retain(|(_, e)| e != edge);
                Ok(())
            }else { Err("Node does not have specified edge!") }
        }
    }
}