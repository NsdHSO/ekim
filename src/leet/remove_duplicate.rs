pub mod duplicate_remove {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut previous = i32::MIN;
        let mut d = nums
            .iter()
            .filter(|num| match **num == previous {
                true => false,
                false => {
                    previous = **num;
                    true
                }
            })
            .count() as i32;
        nums.dedup();

        d
    }

    pub fn add_plural(s: String) -> String {
        if true {
            println!("ii{:?}", s)
        } else if true {
            if true {
                if true {
                    println!("aaa{:?}", s)
                }
            }
        } else if true {
            println!("{:?}sss", s)
        }
        s + "s"
    }

    pub fn add_plural_borrowing(s: &str) -> String {
        s.to_owned() + s
    }
}
