use sha3::{Digest, Sha3_256};

type HashedData = [u8; 32];

pub fn hash(leaves: &[HashedData]) -> HashedData {
    let new_hash = leaves
        .iter()
        .fold(Sha3_256::new(), |mut acc, value| {
            acc.update(value);
            acc
        })
        .finalize().into();

    new_hash
}

pub struct MerkleTree {
    leaves: Vec<Vec<HashedData>>,
}

impl MerkleTree {
    pub fn new<T: std::convert::AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        let initial_blocks: Vec<HashedData> = data
            .iter()
            .map(|value| {
                Sha3_256::digest(value).into()
            })
            .collect();

        let mut tree: Vec<Vec<HashedData>> = vec![];

        let mut new_leaves = initial_blocks;
        tree.push(new_leaves.clone());
        loop {

	  new_leaves = new_leaves
	      .chunks(2)
	      .map(|left_n_right|
		 {
		     if left_n_right.get(1).is_none() {
		        // This is the case where we have an uneven
		        // amount of nodes
		         hash(&[left_n_right[0], left_n_right[0]])
		     } else {
		         hash(&[left_n_right[0], left_n_right[1]])
		     }
		 }
	      )
	      .collect();
	  tree.push(new_leaves.clone());
	  if new_leaves.len() == 1 {
	      break;
	  }
        }

        MerkleTree { leaves: tree }

    }

    pub fn generate_proof<T: std::convert::AsRef<[u8]>>(&self, elem: &T) -> Option<Vec<HashedData>> {

        // It there is no first level, for whatever reason, reaturn None
        let first_level = self.leaves.get(0)?;

        // If the requested item is not present in the data block, we
        // also return None
        let mut index = first_level
	  .iter()
	  .position(|og_data| {
	      let check: HashedData = Sha3_256::digest(elem).into();
	      *og_data == check
	  })?;

        // We need the brother of the requested item.
        let brother = if index % 2 == 0 { index + 1 } else { index - 1 };
        let mut needed_values: Vec<_> = vec![index, brother];

        // The first two values *MUST* be sorted, since the hashing
        // function varies output depending on order.
        needed_values.sort();

        // NOTE: This section is very imperative, I'd like to simplify
        // it later, for now it works. But I am not a fan at all.
        // This is just a WIP
        for _ in self.leaves.iter().skip(1) {
	  let parent_index = (index as f32 / 2.0).floor() as usize;
	  let uncle_index = if parent_index % 2 == 0 { parent_index + 1 } else { parent_index - 1 };

	  index = parent_index;
	  needed_values.push(uncle_index);
        }

        // The last element contains the needed value for the root,
        // which we don't need. Thus, we remove it
        needed_values.remove(needed_values.len() - 1);
        println!("{:?}", needed_values);

        // NOTE: There is no reason for me to go through the vector
        // twice. I could store all the values in one go. I am doing
        // this to simply debugging. 
        let first_value = self.leaves[0][needed_values[0]];
        let mut hashes: Vec<HashedData> = vec![first_value];

        let mut required_index = needed_values.iter().skip(1);

        for level in &self.leaves {
	  if level.len() == 1 {
	      break;
	  }
	  if let Some(index) = required_index.next() {
	      let value = level[*index];
	      hashes.push(value);
	  } else {
	      panic!("INVARIANT HAS BEEN BROKEN. THIS SHOULD NOT HAPPEN")
	  }
        }
        Some(hashes)
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
    fn generate_proof_test() {

        let merkle_tree = MerkleTree::new(&[
	  "0",
	  "1",
	  "2",
	  "3",
	  "4", 
	  "5", 
	  "6", 
	  "7", 
        ]);

        merkle_tree.generate_proof(&"5");
        assert!(false)
    }

}
