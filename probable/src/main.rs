
pub mod merkel_tree{

    // tree props
    #[derive(Debug, Clone)]
      pub struct PrimaryNode{
        pub data : Vec::<String>,
        pub left : Vec::<PrimaryNode>,
        pub right : Vec::<PrimaryNode>,
      }

      // enums tree holding
      pub enum Results{
        Ok,
        Err,
      }


      impl PrimaryNode{

        // tree root node
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


        // climbing on tree whether left or right branch
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


        // new node merage with old one
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


        // either merage on right or left
        pub fn get_ref(&self, block : PrimaryNode) -> Vec::<PrimaryNode>{
          self.merge(block)
        }

    }


      // specialize functions
      pub fn turn(primary : Vec::<PrimaryNode>) -> Vec::<PrimaryNode>{

        match primary{
          data => {data},
          _ => {
            let non_exist = Vec::<PrimaryNode>::new();
            non_exist
          }
        }
    }


    pub fn next(new : PrimaryNode) -> Vec::<PrimaryNode> {
      let mut it = Vec::<PrimaryNode>::new();
      it.push(new);
      it
    }


}


use crate::merkel_tree::*;

fn main() {

    //declaration of data
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


// create a leaves for branches
fn init(this_data: Vec::<String>) -> PrimaryNode{
  let data_block = PrimaryNode{data : this_data,
                                left : Vec::<PrimaryNode>::new(),
                                right: Vec::<PrimaryNode>::new()};
  data_block
}

// dynamic leaves and root nodes
fn new_collect(node : Vec::<String>) -> PrimaryNode{
  init(node)
}
