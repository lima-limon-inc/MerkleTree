use sha3::{Digest, Sha3_256};

type HashedData = [u8; 32];
type Position = usize;

pub fn hash(leaves: &[HashedData]) -> HashedData {
    let new_hash = leaves
        .iter()
        .fold(Sha3_256::new(), |mut acc, value| {
            acc.update(value);
            acc
        })
        .finalize()
        .into();

    new_hash
}

pub struct MerkleTree {
    leaves: Vec<Vec<HashedData>>,
}

impl MerkleTree {
    pub fn new<T: std::convert::AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        let initial_blocks: Vec<HashedData> = data
            .iter()
            .map(|value| Sha3_256::digest(value).into())
            .collect();

        let mut tree: Vec<Vec<HashedData>> = vec![];

        let mut new_leaves = initial_blocks;
        tree.push(new_leaves.clone());
        loop {
            new_leaves = new_leaves
                .chunks(2)
                .map(|left_n_right| {
                    if left_n_right.get(1).is_none() {
                        // This is the case where we have an uneven
                        // amount of nodes
                        hash(&[left_n_right[0], left_n_right[0]])
                    } else {
                        hash(&[left_n_right[0], left_n_right[1]])
                    }
                })
                .collect();
            tree.push(new_leaves.clone());
            if new_leaves.len() == 1 {
                break;
            }
        }

        MerkleTree { leaves: tree }
    }

    // This function returns the index of the hash mainly to make
    // debugging easier. Plus, I thinks it's a cool bonus. It can
    // always be ignored with (_, hash)
    pub fn generate_proof<T: std::convert::AsRef<[u8]>>(
        &self,
        elem: &T,
    ) -> Option<Vec<(Position, HashedData)>> {
        // It there is no first level, for whatever reason, reaturn None
        let first_level = self.leaves.get(0)?;

        // If the requested item is not present in the data block, we
        // also return None
        let mut index = first_level.iter().position(|og_data| {
            let check: HashedData = Sha3_256::digest(elem).into();
            *og_data == check
        })?;

        let mut hashes: Vec<(Position, HashedData)> = vec![];

        for layer in self.leaves.iter() {
            // Skip root
            if layer.len() == 1 {
                break;
            }
            if index % 2 == 0 {
                // Index is even. We have to check if it has a sibling.
                // Example case merkle tree from 0 to 6
                if let Some(right_s) = layer.get(index + 1) {
                    hashes.push((index + 1, *right_s));
                } else {
                    // If the node doesn't have a sibling, that nodes
                    // will be its own sibling.
                    hashes.push((index, layer[index]));
                }
            } else {
                // Odd numbers always have a sibling to the right.
                hashes.push((index - 1, layer[index - 1]));
            }

            index /= 2;
        }

        Some(hashes)
    }

    pub fn verify<T: std::convert::AsRef<[u8]>>(&self, proof: Vec<HashedData>, check: &T) -> bool {
        let check: HashedData = Sha3_256::digest(check).into();

        let mut new_root = check;

        let mut element_index = self
            .leaves
            .get(0)
            .unwrap()
            .iter()
            .position(|og_data| *og_data == check)
            .unwrap();

        // println!("{}", element_index);

        for part in proof {
	  println!("{}", element_index);
            if element_index % 2 == 0 {
                new_root = hash(&[new_root, part]);
            } else {
                new_root = hash(&[part, new_root]);
            }

	  element_index /= 2;
        }

        println!("Root {:?}", self.leaves[self.leaves.len() - 1][0]);
        println!("New root {:?}", new_root);
        new_root == self.leaves[self.leaves.len() - 1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merkel_tree_not_power_2() {
        //        		   11
        //        		   /\
        //        		  /  \
        //        		 /    \
        //        	          /      \
        //        	         /        \
        //        	        /          \
        //        	       /	          \
        //        	      /	           \
        //        	     /	            \
        //        	    /	             \
        //                    9                     10
        //        	  /\	              /\
        //                   /  \	             /  \
        //                  /    \	            /    \
        //                 /      \	           /      \
        //                /	       \	          /        \
        //                6        7            8        8
        //               /\	        /\          /\
        //              /  \       /  \        /  \
        //             /    \     /	 \      /    \
        //            /	 \   /	  \    /      \
        //           /	  \ /	   \  /        \
        //          0         1 2        3 4         5

        let merkle_tree = MerkleTree::new(&["0", "1", "2", "3", "4", "5"]);
        assert_eq!(merkle_tree.leaves.len(), 4)
    }

    #[test]
    fn merkel_tree_power_2() {
        //                    6
        //        	  /\
        //                   /  \
        //                  /    \
        //                 /      \
        //                /	       \
        //                4         5
        //               /\	        /\
        //              /  \       /  \
        //             /    \     /	 \
        //            /	 \   /	  \
        //           /	  \ /	   \
        //          0         1 2        3

        let merkle_tree = MerkleTree::new(&["0", "1", "2", "3"]);
        assert_eq!(merkle_tree.leaves.len(), 3)
    }

    #[test]
    fn generate_proof_not_power_test() {
        let merkle_tree = MerkleTree::new(&[
            "1", "2", "3", "4", "5",
        ]);

        let proof: Vec<Position> = merkle_tree
            .generate_proof(&"5")
            .unwrap()
            .iter()
            .map(|a| a.0)
            .collect();

        assert_eq!(proof, [4, 2, 0]);
    }

    #[test]
    fn generate_proof_power_test() {
        let merkle_tree = MerkleTree::new(&["0", "1", "2", "3", "4", "5", "6", "7"]);

        let proof: Vec<Position> = merkle_tree
            .generate_proof(&"5")
            .unwrap()
            .iter()
            .map(|a| a.0)
            .collect();

        assert_eq!(proof, [4, 3, 0]);
    }

    #[test]
    fn verify_test() {
        let merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);

        let proof: Vec<HashedData> = merkle_tree
            .generate_proof(&"5")
            .unwrap()
            .iter()
            .map(|(_, value)| *value)
            .collect();

        let is_valid = merkle_tree.verify(proof, &"5");
        println!("{}", is_valid);
        assert!(is_valid);
    }
}
