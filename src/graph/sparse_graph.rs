use super::Connections;

#[derive(Debug)]
struct Vertice<T> {
    data: T,
    edges: Vec<usize>, 
}

#[derive(Debug)]
struct SparseGraph<T> {
    verts: Vec<Vertice<T>>,
}

#[derive(Debug)]
pub struct SparceGraphConnections {
    conns: Vec<Vec<usize>>,
}

impl SparceGraphConnections {
    pub fn len(&self) -> usize {
        self.conns.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.conns.is_empty()
    }
}

struct EdgeIter<'s> {
    edges: &'s [usize],
}

impl<'s> Iterator for EdgeIter<'s> {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.edges {
            [fst, ..] => {
                let res = *fst;
                self.edges = &self.edges[1..];
                Some(res)
            },
            [] => None,
        }
    }
}

impl Connections for SparceGraphConnections {
    type EdgeIter<'s> = EdgeIter<'s>;
    
    fn with_count(count: usize) -> Self {
        let conns = vec![vec![]; count];
        Self { conns }
    }
    
    fn connect(&mut self, from: usize, to: usize) -> bool {
        if from < self.len() && to < self.len() {
            self.conns[from].push(to);
            self.conns[to].push(from);
            true
        } else {
            false
        }
    }
    
    fn edges<'s>(&'s self, from: usize) -> Self::EdgeIter<'s> {
        let edges = self.conns.get(from)
            .map(|vec| &vec[..])
            .unwrap_or(&[]);
        EdgeIter { edges }
    }
}
