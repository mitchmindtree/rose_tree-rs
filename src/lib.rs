

extern crate petgraph as pg;


pub use pg as petgraph;
pub use pg::graph::{DefIndex, IndexType, NodeIndex};


/// The PetGraph to be used internally within the RoseTree for storing/managing Nodes and Edges.
pub type PetGraph<N, Ix> = pg::Graph<N, (), pg::Directed, Ix>;


/// An indexable tree data structure with a variable and unbounded number of branches per node.
///
/// Also known as a multi-way tree.
///
/// See the [wikipedia article on the "Rose tree" data
/// structure](https://en.wikipedia.org/wiki/Rose_tree).
///
/// Note: The following documentation is adapted from petgraph's **Graph** documentation.
///
/// **RoseTree** is parameterized over the node weight **N** and **Ix** which is the index type
/// used.
///
/// A wrapper around petgraph's **Graph** data structure with a refined API specifically targeted
/// towards use as a **RoseTree**.
///
/// **NodeIndex** is a type that acts as a reference to nodes, but these are only stable across
/// certain operations. **Removing nodes may shift other indices.** Adding kids to the **RoseTree**
/// keeps all indices stable, but removing a node will force the last node to shift its index to
/// take its place.
///
/// The fact that the node indices in the **RoseTree** are numbered in a compact interval from 0 to
/// *n*-1 simplifies some graph algorithms.
///
/// The **Ix** parameter is u32 by default. The goal is that you can ignore this parameter
/// completely unless you need a very large **RoseTree** -- then you can use usize.
///
/// The **RoseTree** also offers methods for accessing the underlying **Graph**, which can be useful
/// for taking advantage of petgraph's various graph-related algorithms.
#[derive(Clone, Debug)]
pub struct RoseTree<N, Ix: IndexType = DefIndex> {
    /// A graph for storing all Nodes and edges between them that represent the tree.
    graph: PetGraph<N, Ix>,
}


impl<N, Ix = DefIndex> RoseTree<N, Ix> where Ix: IndexType {

    /// Create a new `RoseTree` along with some root Node.
    /// Returns both the `RoseTree` and an index into the root Node in a tuple.
    pub fn new(root: N) -> (Self, NodeIndex<Ix>) {
        Self::with_capacity(1, root)
    }

    /// Create a new `RoseTree` with estimated capacity and some root Node.
    /// Returns both the `RoseTree` and an index into the root Node in a tuple.
    pub fn with_capacity(nodes: usize, root: N) -> (Self, NodeIndex<Ix>) {
        let mut graph = PetGraph::with_capacity(nodes, nodes);
        let root = graph.add_node(root);
        (RoseTree { graph: graph }, root)
    }

    /// The total number of nodes in the RoseTree.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Remove all nodes in the `RoseTree` except for the root.
    pub fn remove_all_but_root(&mut self) {
        // We can assume that the `root`'s index is zero, as it is always the first node to be
        // added to the RoseTree.
        if let Some(root) = self.graph.remove_node(NodeIndex::new(0)) {
            self.graph.clear();
            self.graph.add_node(root);
        }
    }

    /// Borrow the `RoseTree`'s underlying `PetGraph<N, Ix>`.
    /// All existing `NodeIndex`s may be used to index into this graph the same way they may be
    /// used to index into the `RoseTree`.
    pub fn graph_ref(&self) -> &PetGraph<N, Ix> {
        &self.graph
    }

    /// Take ownership of the RoseTree and return the internal PetGraph<N, Ix>.
    /// All existing `NodeIndex`s may be used to index into this graph the same way they may be
    /// used to index into the `RoseTree`.
    pub fn into_graph(self) -> PetGraph<N, Ix> {
        let RoseTree { graph } = self;
        graph
    }

    /// Add a child Node to the Node at the given NodeIndex.
    /// Returns an index into the child's position within the tree.
    ///
    /// Computes in **O(1)** time.
    ///
    /// **Panics** if the given parent node doesn't exist.
    ///
    /// **Panics** if the Graph is at the maximum number of edges for its index.
    pub fn add_child(&mut self, parent: NodeIndex<Ix>, kid: N) -> NodeIndex<Ix> {
        let kid = self.graph.add_node(kid);
        self.graph.add_edge(parent, kid, ());
        kid
    }

    /// Borrow the weight from the node at the given index.
    pub fn node_weight(&self, node: NodeIndex<Ix>) -> Option<&N> {
        self.graph.node_weight(node)
    }

    /// Mutably borrow the weight from the node at the given index.
    pub fn node_weight_mut(&mut self, node: NodeIndex<Ix>) -> Option<&mut N> {
        self.graph.node_weight_mut(node)
    }

    /// Index the `RoseTree` by two node indices.
    ///
    /// **Panics** if the indices are equal or if they are out of bounds.
    pub fn index_twice_mut(&mut self, a: NodeIndex<Ix>, b: NodeIndex<Ix>) -> (&mut N, &mut N) {
        self.graph.index_twice_mut(a, b)
    }

}


