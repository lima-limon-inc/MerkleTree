use sha3::{Digest, Sha3_256};

type Position = usize;

#[derive (Debug)] 
pub struct Leaf {
    // The hash is an array of 32 u8
    hash: [u8; 32],

    // Each leaf will have an index to its children.
    // It's an option type because maybe the child does not
    // exist.
    left_child: Option<Position>,
    right_child: Option<Position>,
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

pub fn hash(positions: [Position; 2], leaves: &[&Leaf; 2]) -> Leaf {
    let new_hash = leaves.iter()
        .fold(Sha3_256::new(), |mut acc, a| {
	  let value = a.hash;
	  acc.update(value);
	  acc}).finalize();

    Leaf {
        hash: new_hash.into(),
        right_child: Some(positions[1]),
        left_child: Some(positions[0]),
    }
}


pub struct MerkleTree {
    leaves: Vec<Leaf>,
}

impl MerkleTree {
    pub fn new<T: std::convert::AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        let original_leaves: Vec<(Position, Leaf)> = data
	  .iter()
	  .map(|value| {
	      let hash = Sha3_256::digest(value);
	      hash
	  })
	  .map(|hash| Leaf::new(hash.into(), None, None))
	  .enumerate()
	  .collect();

        let new_leaves = Self::add_children_leaves(original_leaves);

        todo!();

        // MerkleTree {
        // 	  leaves,
        // }
    }

    fn add_children_leaves(original_leaves: Vec<(Position, Leaf)>) -> Vec<Leaf> {
        let new_leaves = original_leaves
	  // Grab two items at a time
            .chunks(2)
            .into_iter()
            .map(|a| {
	      // This is the case where there is an uneven amount
	      // of data elements. The chunks function will returns
	      // the first element by itself. The second element will be none
	      if a.get(1).is_none() {
		[&a[0],&a[0]]
	      } else {
		[&a[0], &a[1]]
	      }
	      
	  })
            .fold(Vec::new(), |mut acc, x| {
	      let positions = [x[0].0, x[1].0];
	      let leaves = [&x[0].1, &x[1].1];

	      let new_leaf = hash(positions, &leaves);
	      acc.push(new_leaf);
	      acc
	  });

        new_leaves
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: This tests isn't really good.
    #[test]
    fn merkel_tree_new() {
        let merkel = MerkleTree::new(&["90", "98", "89", "92"]);
        for leaf in merkel.leaves {
	  for byte in leaf.hash  { 
	      println!("{}", byte);
	  }
        }
    }
}
