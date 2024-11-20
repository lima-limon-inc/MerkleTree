use sha3::{Keccak256, Digest};

struct Leaf {
    // The hash is an array of 32 u8
    hash: [u8; 32],

    // Each leaf will have an index to its children.
    // It's an option type because maybe the child does not
    // exist.
    left_child: Option<usize>,
    right_child: Option<usize>
}

struct MerkleTree {
    leaves: Vec<Leaf>,
}

pub fn hello_world() {
    let mut hasher = Keccak256::new();
    let data = b"Hello world!";
    println!("Binary hash: {:?}", data);
    hasher.update(data);
    let hash = hasher.finalize();
    println!("Binary hash: {:?}", hash);
}
