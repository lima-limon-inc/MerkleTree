use sha3::{Digest, Sha3_256};

type HashedData = [u8; 32];

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
    pub fn new<T: std::convert::AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        let initial_blocks: Vec<HashedData> = data
            .iter()
            .map(|value| Sha3_256::digest(value).into())
            .collect();

        let tree = Self::generate_tree(initial_blocks);

        MerkleTree { leaves: tree }
    }

    // This function returns the index of the hash mainly to make
    // debugging easier. Plus, I thinks it's a cool bonus. It can
    // always be ignored with (_, hash)
    pub fn generate_proof<T: std::convert::AsRef<[u8]>>(
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

    pub fn verify<T: std::convert::AsRef<[u8]>>(&self, proof: Vec<HashedData>, check: &T) -> bool {
        let check: HashedData = Sha3_256::digest(check).into();

        let mut element_index = self
            .leaves
            .get(0)
            .unwrap()
            .iter()
            .position(|og_data| *og_data == check)
            .unwrap();

        let new_root = proof.iter().fold(check, |mut accumulated_hash, proof| {
            if element_index % 2 == 0 {
                accumulated_hash = hash(&[accumulated_hash, *proof]);
            } else {
                accumulated_hash = hash(&[*proof, accumulated_hash]);
            }
            element_index /= 2;
            accumulated_hash
        });

        if cfg!(test) {
            println!("Root {:?}", self.leaves[self.leaves.len() - 1][0]);
            println!("New root {:?}", new_root);
        }
        new_root == self.leaves[self.leaves.len() - 1][0]
    }

    pub fn add_element<T: std::convert::AsRef<[u8]>>(&mut self, new_val: &T) {
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
    fn generate_proof_not_power_test() {
        let merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);

        let proof: Vec<usize> = merkle_tree
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

        let proof: Vec<usize> = merkle_tree
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

    #[test]
    fn generate_new_tree() {
        let mut merkle_tree = MerkleTree::new(&["1", "2", "3", "4", "5"]);
        merkle_tree.add_element(&"6");

        let proof_pos: Vec<usize> = merkle_tree
            .generate_proof(&"6")
            .unwrap()
            .iter()
            .map(|(position, _)| *position)
            .collect();

        let proof_values: Vec<HashedData> = merkle_tree
            .generate_proof(&"6")
            .unwrap()
            .iter()
            .map(|(_, values)| *values)
            .collect();

        assert_eq!(proof_pos, [4, 2, 0]);
        let is_valid = merkle_tree.verify(proof_values, &"6");
        assert!(is_valid);
    }
}
