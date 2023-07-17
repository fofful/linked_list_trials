use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
   val: i32,
   next: Option<Box<ListNode>>
 }

impl ListNode {
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            val,
            next,
        }
    }

   fn return_integer_value(&self) -> i32 {
    let mut current = self;
    let mut integer_value_str = current.val.to_string();

        while let Some(next) = &current.next{
            integer_value_str = integer_value_str.to_string() + &next.val.to_string();
            current = next;
        }
    let integer_value_int = integer_value_str.parse::<i32>();
    match integer_value_int {
        Ok(value) => return value,
        Err(_) => return -1,
    }
   }

 }

fn parse_str_to_int(str_value: &str) -> i32 {
    let result = str_value.parse::<i32>();
    match result {
        Ok(value) => return value,
        Err(_) => return -1,
    }
}

fn handle_file_to_linked_lists(file_path: &str) -> Vec<ListNode>{
    let mut head_node_vec: Vec<ListNode> = Vec::new();
    let file = File::open(file_path);
    match file{
        Ok(file) => {
            let reader = BufReader::new(file);
            
            for line in reader.lines(){
                let line = line.unwrap();
                let split_string: Vec<&str> = line.split(",").collect();
                for (index, str_entry) in split_string.iter().rev().enumerate() {
                    let int_entry = parse_str_to_int(str_entry);
                    if index == 0 {
                        let new_node = ListNode::new(int_entry, None);
                        head_node_vec.push(new_node);
                    }
                    else {
                        let new_node = ListNode::new(int_entry, Some(Box::new(head_node_vec.pop().unwrap())));
                        head_node_vec.push(new_node);
                    }
                }
            }
        }
        Err(error) => {
            println!("no file {} found, error {}", file_path, error)
        }
    }
    return head_node_vec
}

fn add_vec_values(values_vec: Vec<i32>) -> i32{
    let added_sum: i32 = values_vec.iter().fold(0, |accumulator, &x| accumulator + x);
    return added_sum
}

fn main() {
    let file_path = "./data/arrays.txt"; 
    let head_node_vec: Vec<ListNode> = handle_file_to_linked_lists(file_path);
    let mut values_vec = Vec::new();
    for node in head_node_vec{
        values_vec.push(node.return_integer_value());
    }
    println!("added values: {:?}", values_vec);
    let added_sum = add_vec_values(values_vec);
    println!("added sum: {:?}", added_sum);

}

