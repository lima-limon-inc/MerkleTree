use sha3::{Keccak256, Digest};

type Position = usize;

struct Leaf {
    // The hash is an array of 32 u8
    hash: [u8; 32],

    // Each leaf will have an index to its children.
    // It's an option type because maybe the child does not
    // exist.
    left_child: Option<Position>,
    right_child: Option<Position>,

    // // NOTE: I am NOT a fan of a leaf having it's own index. It seems
    // // like duplicating information, which can lead to false assumptions.
    // // I hope it doesnt bite me in the butt later on.
    // my_position: Position,

}

impl Leaf {
    pub fn new(hash: [u8; 32], left_child: Option<Position>, right_child: Option<Position>) -> Leaf {
        Leaf {
	  hash,
	  left_child,
	  right_child,
        }
    }

}


struct MerkleTree {
    leaves: Vec<Leaf>,
}

pub fn hash(positions: [Position; 2], leaves: [Leaf; 2]) -> Leaf {
    let mut hasher = Keccak256::new();

    for leaf in leaves.iter() {
        hasher.update(leaf.hash);
    }

    let new_hash = hasher.finalize();
    Leaf {
        hash: new_hash.into(),
        right_child: Some(positions[1]),
        left_child: Some(positions[0]),
    }
}
