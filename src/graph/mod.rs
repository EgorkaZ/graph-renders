mod dense_graph;
mod sparse_graph;


pub trait Connections {
    type EdgeIter<'s>: Iterator<Item = usize>;
    
    fn with_count(count: usize) -> Self;

    /// Connects verticies both ways and returns `true`.
    /// If any of the indicies is invalid, does nothing and returns false
    fn connect(&mut self, from: usize, to: usize);    
    
    fn edges<'s>(&'s self, idx: usize) -> Self::EdgeIter<'s>;
}
