use sha3::{Digest, Sha3_256};

type Position = usize;
type HashedData = [u8; 32];

pub fn hash(leaves: &[HashedData]) -> HashedData {
    let new_hash = leaves
        .iter()
        .fold(Sha3_256::new(), |mut acc, a| {
            let value = a;
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
		         hash(&[left_n_right[0]])
                             // [&left_n_right[0], &left_n_right[0]]

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

        println!("HERE");
        println!("{:?}", tree);
        for level in tree {
	  // for node in level {
	  //     print!("{:?}", node);
	  // }
	  println!("{}", level.len())
        }
        todo!();
        // for leaf in tree.clone().iter().flatten().collect() {
        // 	  println!("{:?}", leaf);
        // }


        // MerkleTree { leaves }

    }

    // fn add_children_leaves(original_leaves: Vec<(Position, Leaf)>, length: usize ) -> Vec<(Position, Leaf)> {
    //     let mut amount = length;

    //     let new_leaves = original_leaves
    //         // Grab two items at a time
    //         .chunks(2)
    //         .into_iter()
    //         .map(|position_and_leaf| {
    //             // This is the case where there is an uneven amount
    //             // of data elements. The chunks function will returns
    //             // the first element by itself. The second element will be none
    //             if position_and_leaf.get(1).is_none() {
    //                 [&position_and_leaf[0], &position_and_leaf[0]]
    //             } else {
    //                 [&position_and_leaf[0], &position_and_leaf[1]]
    //             }
    //         })
    //         .fold(Vec::new(), |mut acc, position_and_leaf| {
    //             let positions = [position_and_leaf[0].0, position_and_leaf[1].0];
    //             let leaves = [&position_and_leaf[0].1, &position_and_leaf[1].1];

    //             let new_leaf = hash(positions, &leaves);
    // 	      let new_position = amount;
    // 	      amount += 1;
    //             acc.push((new_position, new_leaf));
    //             acc
    //         });

    //     new_leaves
    // }
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
        assert_eq!(merkle_tree.leaves.len(), 11 + 1)
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
        assert_eq!(merkle_tree.leaves.len(), 6 + 1)
    }
}
