use std::fs::File;
use std::io::{BufRead, BufReader};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
   val: i32,
   next: Option<Box<ListNode>>
 }

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }

    fn add_next(&mut self, val: i32) {
        let new_node = ListNode {
            val,
            next: self.next.take(),
        };
        self.next = Some(Box::new(new_node));
    }

    fn traverse(&self){
        let mut current = self;
        println!("{:?}", current.val);
        while let Some(next) = &current.next{
            println!("{:?}", next.val);
            current = next;
        }
   }
 }

fn parse_str_to_int(str_value: &str) -> i32 {
    let result = str_value.parse::<i32>();
    match result {
        Ok(value) => value,
        Err(_) => -1,
    }
}

fn main() {
    let mut head_node_vec: Vec<ListNode> = Vec::new();
    let file_name = "./data/arrays.txt"; 
    let file = File::open(file_name);
    match file{
        Ok(file) => {
            let reader = BufReader::new(file);
            
            for line in reader.lines(){
                let line = line.unwrap();
                let split_string: Vec<&str> = line.split(",").collect();
                for (index, str_entry) in split_string.iter().rev().enumerate() {
                    let int_entry = parse_str_to_int(str_entry);
                    if index == 0 {
                        let new_node = ListNode{
                            val: int_entry,
                            next: None,
                        };
                        head_node_vec.push(new_node);
                    }
                    else {
                        let new_node = ListNode{
                            val: int_entry,
                            next: Some(Box::new(head_node_vec.pop().unwrap())),
                        };
                        head_node_vec.push(new_node);
                    }
                }
            }
        }
        Err(error) => {
            println!("no file {} found, error {}", file_name, error)
        }
    }
    println!("len of node: {:?}", head_node_vec[0]);
    head_node_vec[0].traverse();
    head_node_vec[1].traverse();
}

