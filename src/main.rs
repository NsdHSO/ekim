mod leet;
use leet::remove_duplicate::duplicate_remove;
fn main() {
  println!("Hello, world!");
  let mut a = Vec::from([3,3,3,4]);
  let v = duplicate_remove::remove_duplicates(&mut a);
  println!("{:?}", v);
  println!("{:?}", a);


}
