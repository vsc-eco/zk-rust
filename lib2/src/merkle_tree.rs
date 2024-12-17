use crate::node::Node;
use crate::hash::Hash;

pub struct MerkleTree<T>
where
    T: Clone + AsRef<[u8]>,
{
    root: Node<T>,
    leaves: Vec<Node<T>>,
}

impl<T> MerkleTree<T>
where 
    T: Clone + AsRef<[u8]>,
{
    // pub fn new()
}


