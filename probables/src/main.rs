

      
  // --snip--
  ///  Tree is a module which contain collection of objects, functions and traits etc.
  /// Tree have static varaibles {Height and Depth}. More Learn about them discussed in static section below 
  pub mod tree{

    /// Height is a static variable. Height maximum space for Bob Family. Bob Family have 6 children, 1 wife  1 himself. Height of Bob Family or Family space will be 8. In Height parents are also include while depth parents are excluded.
    pub static HEIGHT : usize = 5;

    /// Another static variable is Depth. Depth is level pointer. Means If a node name with (Bob) and alice asked how much members in your family. Bob will answer like I have 1 son and a daughter and both have 2 sons and 2 daughters. My  family size will be 6.  
    pub static DEPTH: usize = 0;

    /// Primary Node is an object. Few deriveable traits attached with primary node object. User should implement these traits.
    /// Primary Node have three fields. Data is the first field. 
    /// Primary Node have two node object {Left or Right} 
    /// These nodes are dynamic in nature.

    #[derive(Debug, Clone)]
    pub struct PrimaryNode{

      /// Data field is dynamic arrary of String.
        pub data : Vec::<String>,

     /// Dynamic Array of Node (Node*)
        pub left : Vec::<PrimaryNode>,

     /// Dynamic Array of Node (Node*) 
        pub right :Vec::<PrimaryNode>,
      }

      
      /// Results have only one derivable traits Debug. Results Enumeration are used during executation of a program. 
      /// OK and Error are two enumerate state in any program. When a program fail during executation then program in Error state,
      ///  otherwise program is in other state.
      #[derive(Debug)]
      pub enum Results{
        Ok,
        Err,
      }

      /// Branch Enumeration have three Variants. Branch Enumeration is used when programmer want to play around with tree structure.
      /// Like Result it's also derivable with tarit called Debug. Programer must enforces these traits properties.
      /// Enumerate State are Left, Right & Parent.
      
      #[derive(Debug)]
      pub enum Branch{ 
        /// Likewise parent, left and right are also special variants. These varients acheive only when ceratins statements met. 
        Left,
        Right,
        /// Parent is the special variant. These variants are trigger only when certains statements met. 
        Parent,
      }

      /// Cache Result is an generic object. This object contains in memory object and state which is in String. 
      /// Cache implement clousre. Check the docs if you're curiois in.
      /// This object implement Fn trait.
      struct CachedResult<T> where T: Fn(String)-> String, {
        in_mem : T,
        state : Option<String>,
      }

      /// Cache Result implement Fn trait and receiver. Our CachedResult is generic object which state that receiver in generic state.
      /// Cache Result handle String data as return value.
      impl<T> CachedResult<T> where T: Fn(String)-> String {
        
        /// new is an receiver method which takes in memory generic object.
        /// new receiver return CachedResult Object.   
        fn new(in_mem: T) -> CachedResult<T>{
          CachedResult{
            in_mem,
            state : None,
          }
        }

        /// state is another receiver method which takes in mutable self object and string value.
        /// state is receiver method which return string value.
        fn state (&mut self, arg : String) -> String {
          
          match &self.state {
            
            Some(v) => v.to_string(),
            
            None =>{
                
              let v = (self.in_mem)(arg);
              self.state = Some(v.clone());
              v

            },
          }
        }
      }
      

      // tree attached Tree trait. Traits should be implemented. Traits encapluate program method, receivers. 
      pub trait Tree{

        /// Branch Leaf is a receiver provided by tree module & tree trait. 
        /// Branch Leaf take many parameters as arguments { Muatble self object, Node Object and branch enumerate}
        fn branch_leaf(&mut self, node: PrimaryNode, branch : Branch) -> Vec::<PrimaryNode>;
        
        /// New Branch take mutable object , Node Object , Branch Enumerate and branch index.
        /// New Branch return Vector of Node Object.   
        /// Branch Enumerate State decide where will new branch emeraged.
        /// In Parent case empty branch object will return. 
        fn new_branch(&mut self, node: PrimaryNode, branch : Branch, iter : u8) -> Vec<PrimaryNode>;
        
        /// Depth is a receiver method which takes in mutable self Object and Branch enumerate.
        /// Depth return usize value. Depth return Bob family children and grandchildren. 
        fn depth(&mut self , branch : Branch) -> usize;

        /// Recursive Receiver takes a bunch of arguments as parameters Muatble Self Object, Branch Enumerate and index value. 
        /// Recursive Receiver return value. Recursive Receiver iterates leaf in other words ant walk over leaves.
        fn recursive_leaf(&mut self , branch : Branch, index : usize) -> usize;

        /// Eagleview Reciver takes a bunch of arguments as parameters. Muatble Self Object , Branch Enumerate, Data in string and index as value.
        /// EagleView Receiver return Result Enumerate. Data should not be empty. 
        fn eagleview(&mut self , branch : Branch, data : String, index : usize) -> Results;
      }

      

      /// Primary Node is an Object and tree traits have some receiver methods that ensure encapsulation never compromised. It's similar like classes that in other language.
      impl Tree for PrimaryNode{

        // --snip--
         
        /// Branch Leaf is a receiver provided by tree module & tree trait. 
        /// Branch Leaf take many parameters as arguments { Muatble self object, Node Object and branch enumerate}
        /// Branch Leaf return Vector of Object (dynamic array of node).
        /// Branch Leaf insert new node based on Branch Enumerate State. 
        /// In Parent case empty branch object will return
        fn branch_leaf(&mut self, node: PrimaryNode, branch : Branch) -> Vec::<PrimaryNode>{
          
          
          match branch{
            
            Branch::Left => {self.left.push(node); self.left.clone()},
            
            Branch::Right => {self.right.push(node); self.right.clone()},
            
            _ => {Vec::new()},
          }
        }

        /// New Branch take mutable object , Node Object , Branch Enumerate and branch index.
        /// New Branch return Vector of Node Object.   
        /// Branch Enumerate State decide where will new branch emeraged.
        /// In Parent case empty branch object will return.

        fn new_branch(&mut self, node: PrimaryNode, branch : Branch, iter : u8) -> Vec<PrimaryNode>{
          
          
          match branch{
            
            Branch::Left => {
              
              self.left[iter as usize].left.push(node);
              
              self.left[iter as usize].left.clone()
            
            },
            
            Branch::Right => {
              
              self.right[iter as usize].right.push(node);
              
              self.right[iter as usize].right.clone()
            },
            
            _ => {Vec::new()},
          }
        }
        
        /// Depth is a receiver method which takes in mutable self Object and Branch enumerate.
        /// Depth return usize value. Depth return Bob family children and grandchildren.
          
        fn depth(&mut self , branch : Branch) -> usize {
          
          
          let mut count : usize = 0;

          match branch{
            
            // Parent
            Branch::Parent => {
              
              // valid that data is empty or not;
              if !self.data.is_empty(){

                // update static "DEPTH" value as clousre
                let counter = |x : usize| -> usize {
                  
                  x+1
                
                };

                count = counter(DEPTH);
              
              }else {
                
                // update static "DEPTH" value as clousre
                let counter = |x : usize| -> usize {
                  
                  x
                
                };
                
                count = counter(DEPTH);
              
              }

              count 
            },
            // Left
            Branch::Left => {
              
              // valid left branch 
              if !self.left[count].data.is_empty() {
                
                let ind = 0;
                
                let num = self.left[ind as usize].recursive_leaf(branch, ind);
                    
                let counter = |x : usize| -> usize {
                     
                  x+1

                };
                    
                count = counter(DEPTH + num);
              
              }else {
                
                let counter = |x : usize| -> usize {
                  
                  x
                
                };
                
                count = counter(DEPTH);
              
              }
              
              count 
            },
            
            // Right 
            Branch::Right => {
              
              if !self.right[count].data.is_empty() {
                
                let ind = 0;
                
                let num = self.right[ind as usize].recursive_leaf(branch, ind);
                    
                let counter = |x : usize| -> usize {
                      
                  x+1
                    
                };
                    
                count = counter(DEPTH + num);
              
              }else {
                
                let counter = |x : usize| -> usize {
                 
                  x
                
                };
                
                count = counter(DEPTH);
              
              }
              
              count 
            
            },
          
          }
        
        }

        /// Recursive Receiver takes a bunch of arguments as parameters Muatble Self Object, Branch Enumerate and index value. 
        /// Recursive Receiver return value. Recursive Receiver iterates leaf in other words ant walk over leaves.
        fn recursive_leaf(&mut self , branch : Branch, index : usize) -> usize{

            let mut num : usize;
            
            match branch{
              
              Branch::Left =>{
                
                let counter = |x : usize| -> usize{
                  
                  x
                
                };

                num = counter(self.left[index as usize].left.iter().count());
                
                if num == 0 || num > 0 {
                   
                  num += 1;
                }
                
                num
              },

              Branch::Right => {
                
                let counter = |x : usize| -> usize{
                  
                  x
                
                };

                num = counter(self.right[index as usize].right.iter().count());
                
                if num == 0 || num > 0 {
                   
                  num += 1;
                
                }

                num
              
              },
              
              Branch::Parent => {
                
                index
              
              }
            
            }
        }

        /// Eagleview Reciver takes a bunch of arguments as parameters. Muatble Self Object , Branch Enumerate, Data in string and index as value.
        /// EagleView Receiver return Result Enumerate. Data should not be empty.  
        fn eagleview(&mut self , branch : Branch,data : String, index : usize) -> Results {
          
          if data.is_empty(){
            return Results::Err
          }

          match branch{

            Branch::Parent => {

              let pattern = |x : Vec::<String>, y : String| -> bool{
                
                x.contains(&y)
              
              };

              let cond = pattern(self.data.clone(), data);
              
              if !self.data.is_empty() &&  cond {
                
                Results::Ok
              
              }else{
                
                Results::Err
              
              }
            
            },
            
            Branch:: Left => {
              
              let pattern = |x : Vec::<String>, y : String| -> bool{
                
                x.contains(&y)
              
              };

              let mut heap_slicer = CachedResult::new(|value|{value});

              let mut cond = pattern(self.left[index as usize].data.clone(), heap_slicer.state(data.clone()));
              
              if self.left[index as usize].data.is_empty() && cond {
                
                Results::Err
              
              }else{

                cond = pattern(self.left[index as usize].left[index as usize].data.clone(), heap_slicer.state(data.clone()));
                
                while self.left[index as usize].left[index as usize].data.is_empty() &&  cond{
                  
                  break; 
                
                }

                let mut iterates = self.left[index as usize].recursive_leaf(Branch::Left, index as usize);
                
                while iterates != HEIGHT{
                    
                  if !self.left[index as usize].left[index as usize].data.is_empty() && cond{
                      
                    if self.left[index as usize].left[index as usize].data.contains(&data){
                        
                      return Results::Ok
                      
                    }
                      
                    break;
                  
                  }
                    
                  iterates = iterates + 1;
                
                }
                
                Results::Err
              
              }
            
            },
            
            Branch:: Right => {
              
              let pattern = |x : Vec::<String>, y : String| -> bool{
              
                x.contains(&y)
              
              };

              let mut heap_slicer = CachedResult::new(|value|{value});
                
              let mut cond = pattern(self.right[index as usize].data.clone(), heap_slicer.state(data.clone()));
              
              
              if self.right[index as usize].data.is_empty() && cond {
                
                Results::Err
              
              }else{
                
                cond = pattern(self.right[index as usize].right[index as usize].data.clone(), heap_slicer.state(data.clone()));
                
                while self.right[index as usize].right[index as usize].data.is_empty() &&  cond{
                  
                  break; 
                
                }

                let mut iterates = self.right[index as usize].recursive_leaf(Branch::Right, index as usize);
                
                while iterates != HEIGHT{
                    
                  if !self.right[index as usize].right[index as usize].data.is_empty() && cond{
                      
                    if self.right[index as usize].right[index as usize].data.contains(&data){
                        
                      return Results::Ok
                      
                    }
                      
                    break;
                  
                  }
                    
                  iterates = iterates + 1;
                
                }

                Results::Err
              
              }
            
            }

          }

        }

      }

  }

/// import custom module through crate relative path
use crate::tree::*;

// --snip--

fn main() {

  // create tree structure 
    let mut tree = Vec::<PrimaryNode>::new();
  
    // tree_depth = height;

    // tree counter represent height of the tree
    if tree.iter().count() < HEIGHT   {

      // create a new first leaf genesis   
      tree.push(new_node(String::from("A")));
      
      // create branches 
      let children = new_node(String::from("Aa"));
      let _ = tree[0].branch_leaf(children, Branch::Left);
      
      let children = new_node(String::from("Ab"));
      let _ = tree[0].branch_leaf(children, Branch::Right);
       
      // create leaf node
      let children = new_node(String::from("Aaa"));
      let _ = tree[0].new_branch(children, Branch::Left, 0);

      let children = new_node(String::from("aAa"));
      let _ = tree[0].new_branch(children, Branch::Right, 0);
      
      tree.push(new_node(String::from("B")));
      let children = new_node(String::from("Ba"));
      let _ = tree[1].branch_leaf(children, Branch::Right);

      let children = new_node(String::from("BAa"));
      let _ = tree[1].branch_leaf(children, Branch::Left);

      let children = new_node(String::from("BaA"));
      let _ = tree[1].new_branch(children, Branch::Right, 0);

      // depth function
      let count = tree[0].depth(Branch::Parent);
      let lcount = tree[0].depth(Branch::Left);
      let rcount = tree[0].depth(Branch::Right);

      // bash output
      println!("tree: {:?} Root : {:} Left branch child: {:?} Right branch child: {:?}", tree, count, lcount, rcount);
      
      let inspect = tree[0].eagleview(Branch::Parent, String::from("A"), 0);
      println!("Inspect : {:#?}", inspect);

      // inspector run 
      let child_inspect = tree[0].eagleview(Branch::Left, String::from("Aaa"), 0);
      let child_inspect_last = tree[1].eagleview(Branch::Right, String::from("BaA"), 0);
      
      println!("Left : {:#?}, Right: {:#?}", child_inspect, child_inspect_last);
    }
}


/// New Node is a method that takes a single argument string.
/// New Node return Tree Node Object. 
fn new_node(str : String) -> PrimaryNode{
  
  // create new leaf
  let mut node_data: Vec::<String> = Vec::<String>::new();
  node_data.push(str);

  // assigned child references 
  let child : PrimaryNode = PrimaryNode{data : node_data, 
      left : Vec::<PrimaryNode>::new(), 
      right : Vec::<PrimaryNode>::new()
    };
  child 
}
