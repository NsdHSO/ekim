pub mod duplicate_remove {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut previous = i32::MIN;
    let mut d = nums.iter().filter(
      |num| {
      match **num == previous  {
        true => {
          false
        }
        false => {
          previous = **num;
          true
        }
      }
      }
    ).count() as i32;
    nums.dedup();

    d
  }

  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

  }
}