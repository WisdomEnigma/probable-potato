

  //! # probable
  //! probable data structure crate that is purely written in rust.

      //! # MerkelTree
  //! Merkel Tree is the module that i have written, however my work is not yet completed. I will add my features that will update crate and reduce some complexities and improve cratebase

  // --snip--
  pub mod merkel_tree{

    /// tree props
    #[derive(Debug, Clone)]
      pub struct PrimaryNode{

      /// data attribute: this branch hold some leaves. Each leave have own information
        pub data : Vec::<String>,

     /// left attribute: this branch have connection with other branch of tree but which are only on left side
        pub left : Vec::<PrimaryNode>,

     /// right attribute: this branch have connection with other branch of tree but which are only on right side
        pub right : Vec::<PrimaryNode>,
      }

      /// enums are helpful because, it tells us why program fail ?
      pub enum Results{
        Ok,
        Err,
      }



      /// "PrimaryNode" is an structure data. impl provide special functions that are helpful for this data such as root, left or right
      impl PrimaryNode{
        // --snip--
        /// programmer will all the roots data there may be possible there is no data then error report
        pub fn root(&self) -> (Vec::<String>, Results){
          match self{
            data => {
                  (self.data.clone(), Results::Ok)
            },
            _ => {
              let non_exist = Vec::<String>::new();
              (non_exist, Results::Err)
              }
            }
        }


        /// each branch hold some data and our agent move between branches. walk is like gps functions
        pub fn walk(&self) -> (Vec::<PrimaryNode>, Results){
          match self{
            left => {
              (turn(self.left.clone()), Results::Ok)
            }
            right => {
              (turn(self.right.clone()), Results::Ok)
            }
          }
        }


        /// merge are where the new leaf attached (which branch)
        pub fn merge(&self, block : PrimaryNode) -> Vec::<PrimaryNode>{
          match self {
            left =>{
              next(block)
              },
              right =>{
                next(block)
              }
            }
          }


        /// this function is the messenger between user choice and tree decision system.
        pub fn get_ref(&self, block : PrimaryNode) -> Vec::<PrimaryNode>{
          self.merge(block)
        }

    }


      /// does agent take turn or not
      pub fn turn(primary : Vec::<PrimaryNode>) -> Vec::<PrimaryNode>{

        match primary{
          data => {data},
          _ => {
            let non_exist = Vec::<PrimaryNode>::new();
            non_exist
          }
        }
    }


    /// decision system release leaf attachement code
    pub fn next(new : PrimaryNode) -> Vec::<PrimaryNode> {
      let mut it = Vec::<PrimaryNode>::new();
      it.push(new);
      it
    }


}


use crate::merkel_tree::*;

// --snip--
fn main() {

  // declaration of data
  let mut genesis = Vec::<String>::new();
  genesis.push(String::from("hello"));

  let mut second = Vec::<String>::new();
  second.push(String::from("world"));

  let mut third = Vec::<String>::new();
  third.push(String::from("data complete"));


  // dynamic array for tree
  let mut collect = Vec::<PrimaryNode>::new();
  collect.push(new_collect(genesis));
  collect.push(new_collect(second));
  collect.push(new_collect(third));


 // get tree child data
  for i in collect.iter(){

    let (data, _) = i.root();
    println!("Root data: ({:#?})", data);

  }

  // where new branch created
  collect[0].left = collect[0].get_ref(collect[1].clone());
  collect[0].right = collect[0].get_ref(collect[2].clone());

  println!("collection: ({:#?})", collect);

  // climbing on data branches
  for i in collect.iter(){
    let (walk, _) = i.walk();
    println!("walk: ({:#?})", walk);
  }

}


/// create root node
fn init(this_data: Vec::<String>) -> PrimaryNode{
  let data_block = PrimaryNode{data : this_data,
                                left : Vec::<PrimaryNode>::new(),
                                right: Vec::<PrimaryNode>::new()};
  data_block
}

/// dynamic call about task
fn new_collect(node : Vec::<String>) -> PrimaryNode{
  init(node)
}
