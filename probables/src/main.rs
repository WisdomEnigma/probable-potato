

      
  // --snip--
  ///  Tree is common data structure that are used to represent data ... i.e file or directory
  ///  Tree module is not generic , currently it will allow string type data. But soon we will translate into generics
  pub mod tree{

    // tree public and static attributes {"Height", "Depth"}  
    pub static HEIGHT : usize = 5;

    pub static DEPTH: usize = 0;

    /// PrimaryNode (Branch) is a connector which connect leaves. Branch hold leaves and ants move easily.
    /// Currently backtracking is not implemented  
    #[derive(Debug, Clone)]
    pub struct PrimaryNode{

      /// "Data" what you want to store 
        pub data : Vec::<String>,

     /// "Left" Branch position Reference
        pub left : Vec::<PrimaryNode>,

     /// "Right" Branch position Reference
        pub right :Vec::<PrimaryNode>,
      }

      
      /// Results "Enums" either Ok , Err
      /// Ok => Data validation
      /// Err => Data validation failure
      #[derive(Debug)]
      pub enum Results{
        Ok,
        Err,
      }

      ///  Branch "Enums" have three states either {
      ///   "Parent" => Leave attached with tip of branch
      ///   "Left" => Leave attached with left side of branch
      ///   "Right" => Leave attached with right side of branch
      /// }
      #[derive(Debug)]
      pub enum Branch{
        Left,
        Right,
        Parent,
      }

      /// CacheResults return data quickly. CacheResults you don't need to compute everytime
      struct CachedResult<T> where T: Fn(String)-> String, {
        in_mem : T,
        state : Option<String>,
      }

      /// CachedResult "impl" provide in memory functionality for both reading and writing purpose.
      impl<T> CachedResult<T> where T: Fn(String)-> String {
        fn new(in_mem: T) -> CachedResult<T>{
          CachedResult{
            in_mem,
            state : None,
          }
        }

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
      

      // All the functions are required for tree implementations. That why traits join tree army.
      pub trait Tree{

        fn branch_leaf(&mut self, node: PrimaryNode, branch : Branch) -> Vec::<PrimaryNode>;
        fn new_branch(&mut self, node: PrimaryNode, branch : Branch, iter : u8) -> Vec<PrimaryNode>;
        
        fn depth(&mut self , branch : Branch) -> usize;
        fn recursive_leaf(&mut self , branch : Branch, index : usize) -> usize;

        fn inspect(&mut self , branch : Branch, data : String, index : usize) -> Results;
      }

      

      /// Primary Node "implementator" enforced that tree traits are implemented
      impl Tree for PrimaryNode{

        // --snip--
         
        /// Where the new leaf attached 
        fn branch_leaf(&mut self, node: PrimaryNode, branch : Branch) -> Vec::<PrimaryNode>{
          match branch{
            Branch::Left => {self.left.push(node); self.left.clone()},
            Branch::Right => {self.right.push(node); self.right.clone()},
            _ => {Vec::new()},
          }
        }

        /// Create a new tree branches ; if branch is neither left or right then return empty tree  

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
        
        /// depth of the tree, Depth is the number of branch either on right or left side.
        
        fn depth(&mut self , branch : Branch) -> usize {
          
          let mut count : usize = 0;

          match branch{
            Branch::Parent => {
              if !self.data.is_empty(){
                // count = 0;

                let counter = |x : usize| -> usize {
                  x+1
                };
                count = counter(DEPTH);
              }else {
                let counter = |x : usize| -> usize {
                  x
                };
                count = counter(DEPTH);
              }
              count 
            },
            Branch::Left => {
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

        // functions defined
        // recursion leaf ant walk over the branch & find leaf branch 
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

        // inspect act as tofind method. User want to know does data exist or not. insoect return Results Enum
        fn inspect(&mut self , branch : Branch,data : String, index : usize) -> Results {
          
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
      
      let inspect = tree[0].inspect(Branch::Parent, String::from("A"), 0);
      println!("Inspect : {:#?}", inspect);

      // inspector run 
      let child_inspect = tree[0].inspect(Branch::Left, String::from("Aaa"), 0);
      let child_inspect_last = tree[1].inspect(Branch::Right, String::from("BaA"), 0);
      
      println!("Left : {:#?}, Right: {:#?}", child_inspect, child_inspect_last);
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
