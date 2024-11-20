mod merkle_tree;

fn main() {
    println!("Hello, world!");
    let merkle_tree = merkle_tree::MerkleTree::new(&["90", "98", "89", "92", "78"]);
}
