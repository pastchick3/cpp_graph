#![crate_type = "lib"]

use libc::{c_int, c_void, size_t};

type GraphPtr = *mut c_void;

#[repr(C)]
struct Edge {
    to: size_t,
    weight: c_int,
}

#[repr(C)]
struct Edges {
    array: *mut Edge,
    size: size_t,
}

#[repr(C)]
struct Vertices {
    array: *mut size_t,
    size: size_t,
}

#[link(name = "graph", kind = "static")]
extern "C" {
    fn graph_ctor() -> GraphPtr;
    fn graph_dtor(graph_ptr: GraphPtr);
    fn graph_insert_vertex(graph_ptr: GraphPtr, vertex: size_t);
    fn graph_insert_edge(graph_ptr: GraphPtr, from: size_t, to: size_t, weight: c_int);
    fn graph_remove_vertex(graph_ptr: GraphPtr, vertex: size_t);
    fn graph_remove_edge(graph_ptr: GraphPtr, from: size_t, to: size_t);
    fn graph_get_vertex_number(graph_ptr: GraphPtr) -> size_t;
    fn graph_get_edge_number(graph_ptr: GraphPtr) -> size_t;
    fn graph_get_vertices(graph_ptr: GraphPtr) -> *const Vertices;
    fn graph_free_vertices(vertices: *const Vertices);
    fn graph_get_adjacent_edges(graph_ptr: GraphPtr, vertex: size_t) -> *const Edges;
    fn graph_free_edges(edges: *const Edges);
}

pub struct CppGraph {
    graph: GraphPtr,
}

impl Drop for CppGraph {
    fn drop(&mut self) {
        unsafe {
            graph_dtor(self.graph);
        }
    }
}

impl Default for CppGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl CppGraph {
    pub fn new() -> Self {
        unsafe {
            CppGraph {
                graph: graph_ctor(),
            }
        }
    }

    pub fn insert_vertex(&self, vertex: usize) {
        unsafe { graph_insert_vertex(self.graph, vertex) }
    }

    pub fn insert_edge(&self, from: usize, to: usize, weight: i32) {
        unsafe { graph_insert_edge(self.graph, from, to, weight) }
    }

    pub fn remove_vertex(&self, vertex: usize) {
        unsafe { graph_remove_vertex(self.graph, vertex) }
    }

    pub fn remove_edge(&self, from: usize, to: usize) {
        unsafe { graph_remove_edge(self.graph, from, to) }
    }

    pub fn get_vertex_number(&self) -> usize {
        unsafe { graph_get_vertex_number(self.graph) }
    }
    pub fn get_edge_number(&self) -> usize {
        unsafe { graph_get_edge_number(self.graph) }
    }

    pub fn get_vertices(&self) -> Vec<usize> {
        unsafe {
            let vertices = graph_get_vertices(self.graph);
            let mut vec = Vec::new();
            for i in 0..(*vertices).size {
                let ptr = (*vertices).array.add(i);
                vec.push(*ptr);
            }
            graph_free_vertices(vertices);
            vec
        }
    }

    pub fn get_adjacent_edges(&self, vertex: usize) -> Vec<(usize, i32)> {
        unsafe {
            let edges = graph_get_adjacent_edges(self.graph, vertex);
            let mut vec = Vec::new();
            for i in 0..(*edges).size {
                let ptr = (*edges).array.add(i);
                vec.push(((*ptr).to, (*ptr).weight));
            }
            graph_free_edges(edges);
            vec
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph() {
        let graph = CppGraph::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_vertex(3);
        graph.insert_vertex(4);
        graph.insert_edge(1, 2, 2);
        graph.insert_edge(2, 3, 1);
        graph.insert_edge(3, 4, 1);
        graph.insert_edge(4, 1, 1);

        assert_eq!(graph.get_vertex_number(), 4);
        assert_eq!(graph.get_edge_number(), 4);

        graph.remove_vertex(3);
        graph.remove_edge(4, 1);

        assert_eq!(graph.get_vertex_number(), 3);
        assert_eq!(graph.get_edge_number(), 1);

        let vertices = graph.get_vertices();
        assert_eq!(vertices.len(), 3);

        let edges = graph.get_adjacent_edges(1);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].0, 2);
        assert_eq!(edges[0].1, 2);
    }
}
