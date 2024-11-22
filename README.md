# Merkle Tree 
Simple [Merkle Tree](https://en.wikipedia.org/wiki/Merkle_tree) implementation in rust.

## Installation instructions

1. Clone the repo to your machine. (NOTE: This will only clone the latest commit. This is done in order to reduce disk usage).
```shell
git clone --depth=1 git@github.com:lima-limon-inc/MerkleTree.gitMerkleTreeRust/ 
```

2. Enter the new directory
```shell
cd MerkleTreeRust/
```

3. Use `make` to compile the project and run it.
```shell
make
make run
```
## Usage
This library can be used with any type that implements the `AsRef<[u8]>` trait. 

Fist, you need to create the actual Merkle Tree with a given array of values. These values will serve as the foundations of the Merkle Tree and will remain at the bottom layer.

``` rust
fn main() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
}
```

Once created, one of the main things you can do is generate a proof for a given element. For instance, one could generate a proof for the element `&1`

``` rust
fn main() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
        
        let proof = merkle_tree.generate_proof(&"1");
}
```

This proof will also contain the indexes at each level of the generated proof. This information is not necessary for the proof, and is returned mainly for debugging purposes. It's probably best to remove it. That can be done with the following:

``` rust
fn main() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
        
        let proof = merkle_tree.generate_proof(&"1").unwrap().iter().map(|(_, value)| *value).collect();
}
```

Now you have the proof. This proof allows us to check wether a certain value is present in the tree, without needing to check the entire tree, you only need to compare with the root of the original MerkleTree. This comparisson is done inside the method, like so. 

``` rust
fn main() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
        
        let proof = merkle_tree.generate_proof(&"1").unwrap().iter().map(|(_, value)| *value).collect();
        
        let exists = merkle_tree.verify(proof, &"1");
        println!("{}", exists);
}
```

If all goes well, you should see "true" printed in your terminal.


Furthermore, you can extend the tree to your heart's content. This will modify the original tree and thus, its root. Previously generated proofs will no longer be valid, but new ones will.

``` rust
fn main() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
        
        let proof = merkle_tree.generate_proof(&"1").unwrap().iter().map(|(_, value)| *value).collect();
        
        merkle_tree.add_element(&"6");
        // After this addition, the proof becomes invalid. But new ones can be generated

        let new_proof = merkle_tree.generate_proof(&"1").unwrap().iter().map(|(_, value)| *value).collect();
        
        let exists = merkle_tree.verify(new_proof, &"1");
        println!("{}", exists);
}
```
