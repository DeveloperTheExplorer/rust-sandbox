pub mod leetcode {

    // let mut nums = vec![1, 1, 2];
    // let len: i32 = nums.len() as i32;
    // let k = solution::remove_duplicates(&mut nums);
    use std::collections::HashMap;

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len: i32 = nums.len() as i32;
        if len <= 1 {
            return len;
        }

        let mut j = 0;
        for i in 0..(len as usize) {
            if nums[j] != nums[i] {
                j += 1;
                nums[j] = nums[i];
            }
        }
        (j + 1) as i32
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|num| *num != val);

        nums.len() as i32
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return false;
        }

        let mut nums_tracker: HashMap<i32, u8> = HashMap::new();

        for ele in nums {
            match nums_tracker.insert(ele, 1) {
                Some(_) => return true,
                None => {}
            }
        }

        return false;
    }

    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut double_nums: Vec<i32> = vec![];
        let len = nums.len();

        for i in 0..(len * 2) {
            double_nums.push(nums[i % len]);
        }

        return double_nums;
    }

    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut sum = 0;

        for ele in operations {
            match ele.as_str() {
                "D" => {
                    stack.push(stack.last().unwrap() * 2);
                }
                "C" => {
                    stack.pop();
                }
                "+" => stack.push(stack.last().unwrap() + stack[stack.len() - 2]),
                (str_num) => {
                    stack.push(str_num.parse().unwrap());
                }
            }
        }
        for ele in stack {
            sum += ele;
        }

        return sum;
    }

    pub fn parenthesis_is_valid(s: String) -> bool {
        let mut p_stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                ')' => {
                    let last_char = p_stack.pop();
                    match last_char {
                        Some('(') => {}
                        _ => return false,
                    }
                }
                ']' => {
                    let last_char = p_stack.pop();
                    match last_char {
                        Some('[') => {}
                        _ => return false,
                    }
                }
                '}' => {
                    let last_char = p_stack.pop();
                    match last_char {
                        Some('{') => {}
                        _ => return false,
                    }
                }
                _ => p_stack.push(c),
            }
        }

        return p_stack.len() == 0;
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_arr: Vec<char> = s.chars().collect();

        for ele in t.chars() {
            let index_of_char = s_arr.iter().position(|c| *c == ele);

            match index_of_char {
                Some(index) => {
                    s_arr.remove(index);
                }
                None => return false,
            }
        }

        return s_arr.len() == 0;
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        return nums;
    }
}

// fn main() {
//     let mut nums = vec![1, 1, 2];
//     let k = remove_duplicates(&mut nums);

//     println!("{k} elements removed, nums: {:?}", nums);
// }

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || &val <= self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();

        if &val == self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().unwrap().clone()
    }
}
