use sha3::{Digest, Sha3_256};

type Position = usize;

pub struct Leaf {
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

pub fn hash(positions: [Position; 2], leaves: [Leaf; 2]) -> Leaf {
    let mut hasher = Sha3_256::new();

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


struct MerkleTree {
    leaves: Vec<Leaf>,
}

impl MerkleTree {
    pub fn new<T: std::convert::AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        let leaves: Vec<_> = data
	  .iter()
	  .map(|value| {
	      let hash = Sha3_256::digest(value);
	      hash
	  })
	  .map(|hash| Leaf::new(hash.into(), None, None))
	  .collect();
        MerkleTree {
	  leaves,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: This tests isn't really good.
    #[test]
    fn merkel_tree_new() {
        let merkel = MerkleTree::new(&["90", "98", "89"]);
        for leaf in merkel.leaves {
	  for byte in leaf.hash  { 
	      println!("{}", byte);
	  }
        }
    }
}
