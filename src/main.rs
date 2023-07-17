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

    fn traverse(&self){
        let mut current = self;
        println!("{:?}", current.val);
        while let Some(next) = &current.next{
            println!("{:?}", next.val);
            current = next;
        }
        println!("\n");
   }
 }

fn parse_str_to_int(str_value: &str) -> i32 {
    let result = str_value.parse::<i32>();
    match result {
        Ok(value) => value,
        Err(_) => -1,
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

fn main() {
    
    let file_path = "./data/arrays.txt"; 
    let head_node_vec: Vec<ListNode> = handle_file_to_linked_lists(file_path);
    
    println!("len of node: {:?}", head_node_vec.len());
    head_node_vec[0].traverse();
    head_node_vec[1].traverse();
    head_node_vec[2].traverse();

}

