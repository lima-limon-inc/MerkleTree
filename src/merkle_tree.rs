use sha3::{Digest, Sha3_256};

pub type HashedData = [u8; 32];

/// This function generates the joint hash of two (or more) given hashes
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

/// This struct contains the MerkleTree. This is a tree that has n levels
/// the first level contains the hashes of the original data; whilst
/// the other levels contains the accumulated hash of the bottom two
/// levels. If there's an uneven amount of blocks in a level then the
/// last available block is used to create the upper block
pub struct MerkleTree {
    leaves: Vec<Vec<HashedData>>,
}

impl MerkleTree {
    /// This auxilary function is used to generate MerkleTree's internal
    /// structure given a vector of hashed blocks. These blocks will
    /// serve as the foundation of the MerkleTree. These initial blocks
    /// will remain in the bottom vector.
    fn generate_tree(initial_hashes: Vec<HashedData>) -> Vec<Vec<HashedData>> {
        let mut tree: Vec<Vec<HashedData>> = vec![];
        let mut new_leaves = initial_hashes;
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

        tree
    }

    /// This function will create the entire MerkleTree given a slice of
    /// elements that implement the AsRef<[u8]> trait.
    pub fn new<T: AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        let initial_blocks: Vec<HashedData> = data
            .iter()
            .map(|value| Sha3_256::digest(value).into())
            .collect();

        let tree = Self::generate_tree(initial_blocks);

        MerkleTree { leaves: tree }
    }

    /// This function generates a proof for a given element of the
    /// MerkleTree. If the element is not present in the MerkleTree it
    /// will return None.  The returned vector will contain both the
    /// Position of the element in its respective level and the actual
    /// hash. The position is returned mainly to make debugging
    /// easier.  It can always be ignored with (_, hash)
    pub fn generate_proof_internal<T: AsRef<[u8]>>(
        &self,
        elem: &T,
    ) -> Option<Vec<(usize, HashedData)>> {
        // It there is no first level, for whatever reason, reaturn None
        let first_level = self.leaves.get(0)?;

        // If the requested item is not present in the data block, we
        // also return None
        let mut index = first_level.iter().position(|og_data| {
            let check: HashedData = Sha3_256::digest(elem).into();
            *og_data == check
        })?;

        // Remove the root layer.
        let proof_hashes = self.leaves.iter().filter(|layer| layer.len() > 1).fold(
            Vec::new(),
            |mut hashes, layer| {
                if index % 2 == 0 {
                    if let Some(right_s) = layer.get(index + 1) {
                        hashes.push((index + 1, *right_s));
                    } else {
                        // If the node doesn't have a sibling, that nodes
                        // will be its own sibling.
                        hashes.push((index, layer[index]));
                    }
                } else {
                    hashes.push((index - 1, layer[index - 1]));
                }

                index /= 2;
                hashes
            },
        );
        Some(proof_hashes)
    }

    /// This function receives a proof (generated from the
    /// [`MerkleTree::generate_proof_internal()`] for example. It will return boolean
    /// stating wether the merkle tree contains the value `check`.
    pub fn verify<T: AsRef<[u8]>>(&self, proof: Vec<HashedData>, check: &T) -> bool {
        let check: HashedData = Sha3_256::digest(check).into();

        let Some(first_level) = self.leaves.get(0) else {
            return false;
        };

        let Some(mut element_index) = first_level.iter().position(|og_data| *og_data == check)
        else {
            return false;
        };

        let new_root = proof.iter().fold(check, |mut accumulated_hash, proof| {
            if element_index % 2 == 0 {
                accumulated_hash = hash(&[accumulated_hash, *proof]);
            } else {
                accumulated_hash = hash(&[*proof, accumulated_hash]);
            }
            element_index /= 2;
            accumulated_hash
        });

        // This will only print when testing. Useful to check results
        // in case test fails.
        if cfg!(test) {
            println!("Root {:?}", self.leaves[self.leaves.len() - 1][0]);
            println!("New root {:?}", new_root);
        }
        new_root == self.leaves[self.leaves.len() - 1][0]
    }

    /// This functions adds an element to the MerkleTree. Said value
    /// will be added to the bottom layer all the way to the right
    pub fn add_element<T: AsRef<[u8]>>(&mut self, new_val: &T) {
        let mut initial_blocks: Vec<HashedData> = self.leaves[0].clone();
        let new_value = Sha3_256::digest(new_val).into();
        initial_blocks.push(new_value);
        let new_tree = Self::generate_tree(initial_blocks);
        self.leaves = new_tree;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merkel_tree_not_power_2() {
        let merkle_tree = MerkleTree::new(&["0", "1", "2", "3", "4", "5"]);
        assert_eq!(merkle_tree.leaves.len(), 4)
    }

    #[test]
    fn merkel_tree_power_2() {
        let merkle_tree = MerkleTree::new(&["0", "1", "2", "3"]);
        assert_eq!(merkle_tree.leaves.len(), 3)
    }

    #[test]
    fn generate_proof_internal_not_power_test() {
        let merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);

        let proof: Vec<usize> = merkle_tree
            .generate_proof_internal(&"5")
            .unwrap()
            .iter()
            .map(|a| a.0)
            .collect();

        assert_eq!(proof, [4, 2, 0]);
    }

    #[test]
    fn generate_proof_internal_power_test() {
        let merkle_tree = MerkleTree::new(&["0", "1", "2", "3", "4", "5", "6", "7"]);

        let proof: Vec<usize> = merkle_tree
            .generate_proof_internal(&"5")
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
            .generate_proof_internal(&"5")
            .unwrap()
            .iter()
            .map(|(_, value)| *value)
            .collect();

        let is_valid = merkle_tree.verify(proof, &"5");
        println!("{}", is_valid);
        assert!(is_valid);
    }

    #[test]
    fn generate_new_tree() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
        merkle_tree.add_element(&"6");

        let proof_pos: Vec<usize> = merkle_tree
            .generate_proof_internal(&"6")
            .unwrap()
            .iter()
            .map(|(position, _)| *position)
            .collect();

        let proof_values: Vec<HashedData> = merkle_tree
            .generate_proof_internal(&"6")
            .unwrap()
            .iter()
            .map(|(_, values)| *values)
            .collect();

        assert_eq!(proof_pos, [4, 2, 0]);
        let is_valid = merkle_tree.verify(proof_values, &"6");
        assert!(is_valid);
    }
}
