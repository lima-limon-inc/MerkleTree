mod merkle_tree;

fn main() {
    println!("Hello, world!");
    let merkle_tree = merkle_tree::MerkleTree::new(&[
					  "0",
					  "1",
					  "2",
					  "3",
					  "4",
					  "5",
					   ]);
}
