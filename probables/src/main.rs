

      
  // --snip--
  ///  This module are designed to implement tree structure. Currently it provide very basic functionality  
  pub mod tree{

    /// tree props implemented
    #[derive(Debug, Clone)]
      pub struct PrimaryNode{

      /// data attribute: this branch hold some leaves. Each leave have own information
        pub data : Vec::<String>,

     /// left attribute: this branch have connection with other branch of tree but which are only on left side
        pub left : Vec::<PrimaryNode>,

     /// right attribute: this branch have connection with other branch of tree but which are only on right side
        pub right :Vec::<PrimaryNode>,
      }

      
      /// enums are helpful because, it tells us why program fail ?
      #[derive(Debug)]
      pub enum Results{
        Ok,
        Err,
      }

      /// Either a branch is left one or right one 
      /// Imagine binary tree 
      #[derive(Debug)]
      pub enum Branch{
        Left,
        Right,
        Parent,
      }

      
      

      /// "PrimaryNode" is an structure data. impl provide special functions that are helpful for this data such as root, left or right
      impl PrimaryNode{
        
        // --snip--
        // leaf provide basic functionality for branch 
        // tree have either left or right branch
        // leaves on the branches grow with branch label 
        pub fn leaf(&mut self, node: PrimaryNode, branch : Branch) -> Vec::<PrimaryNode>{
          match branch{
            Branch::Left => {self.left.push(node); self.left.clone() },
            Branch::Right => {self.right.push(node); self.right.clone()},
            _ => {Vec::new()},
          }
        }
      }
    }

use crate::tree::*;

// --snip--
fn main() {

  // create tree structure 
    let mut tree : Vec::<PrimaryNode> = Vec::<PrimaryNode>::new();
  
  // tree vector have finite length   
    let height : usize = 5;

    // tree counter represent height of the tree
    if tree.iter().count() < height   {

      // create a new first leaf genesis   
      tree.push(new_node(String::from("hello")));
      
      let children = new_node(String::from("world"));
      let _ = tree[0].leaf(children, Branch::Left);
      
      let children = new_node(String::from("$$"));
      let _ = tree[0].leaf(children, Branch::Right);
      println!("Tree: {:?}", tree);    
    }
}


// create a new leaf
fn new_node(str : String) -> PrimaryNode{
  
  let mut node_data: Vec::<String> = Vec::<String>::new();
  node_data.push(str);

  let child : PrimaryNode = PrimaryNode{data : node_data, 
      left : Vec::<PrimaryNode>::new(), 
      right : Vec::<PrimaryNode>::new()
    };
  child 
}

  