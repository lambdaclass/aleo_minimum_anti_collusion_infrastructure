#![allow(clippy::module_inception)]
pub mod merkle_tree;
pub use merkle_tree::generate_merkle_root;
pub use merkle_tree::hash;
pub use merkle_tree::MerkleTree;
