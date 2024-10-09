use std::fs::read_to_string;
use std::collections::HashMap;

use chrono::Local;

fn main() {
    string_slice();
}

fn string_slice(){
    let string_Word = String::from("Vishnu"); // Normal String
    let str_word = &string_Word[0..6]; // It's a view of the original string & str_word points over the actual string  with shrinked length;
    let str_literal = "Vishnu"; // It's also type &str, but points to actual reference in binary
}

fn adapter_assignment(){
    //filter out all odds and double each

    let vec = vec![1,2,3];
    let iter = vec.iter();

    let final_iter = iter.filter(|x| **x % 2 != 0 );
    let map_iter = final_iter.map(|x| x*2);

    let final_vec:Vec<i32>  = map_iter.collect();

    println!("The filtered and transformed vector is : {:?}", final_vec);

}

fn iterator_adapter(){
    //The adapter produces another iterator by changing the original adapter
    //The original iterator gets moved just like the consuming adapter
    let v1 = vec![1,2,3];
    let iter = v1.iter();

    // similar to map in js
    // let map_iter = iter.map(|x| x+1);
    // for i in map_iter{
    //     println!("{}", i); // 2 , 3, 4
    // }

    //similar 
    let filter_iter = iter.filter(|x| **x % 2 == 0); 
    for i in filter_iter{
        println!("{}",i); // prints -> 2
    }
}

fn consuming_adapter(){
    // This adapter consumes the original iterator
    // Thus the iterator get's moved to the adapter;

    let v1 = vec![1,2,3];
    let iter = v1.iter();

    let sum : i32 = iter.sum();

    println!("{}",sum); // -> 6

    // println!("{:?}",iter); // -> ERROR : BORROW OF MOVED VALUE

}

fn into_iter_logic(){

    // This is the default version of for loop in rust

    let mut vector = vec![1,2,3,4];

    let iter = vector.into_iter();

    for value in iter{
        println!("The value is {}", value);
    }

    // println!("{:?}",vector); // This complaints as the ownership is moved to iter
}

fn iterator_logic(){
    let mut vector = vec![1,2,3,4];

    // Getting the iterator as mutable
    let mut iter = vector.iter_mut();

    while let Some(val) = iter.next() {
        *val = *val + 1;
        println!("{}",val);
    } 
    
    // let non_mut_iterator = vector.iter();

    // for i in non_mut_iterator{
    //     println!("{} is in non mut iterator",i);
    // }

    // let mut mut_iterator = vector.iter_mut();


    // for i in mut_iterator{
    //     println!("{} is in mut iterator",i);
    // }



}

fn grp_by_tuples(vector:&Vec<(String, u32)>) -> HashMap<String,Vec<u32>> {

    let mut map:HashMap<String,Vec<u32>> = HashMap::new();

    for (k,val) in vector{
        
        let existing_vec: Option<&mut Vec<u32>> = map.get_mut(k);

        match existing_vec {
            Some(arr) => {
                arr.push(*val);
            },
            None => {
                let mut vec: Vec<u32> = vec![*val];
                map.insert(k.clone(), vec);
            }
        }

    }   

    map

}

enum Shape{
    Rect(u32,u32),
    Circle(u32),
}


fn find_first_a(str:String) -> Option<u32>{
    for (ind,c) in str.chars().enumerate(){
        if c == 'a' {
            return Some(ind as u32)
        }
    }
    None
}

fn find_area(shape:Shape) -> u32{
    let area = match shape {
        Shape::Circle(radius) => {
            return  2 * radius;
        },
        Shape::Rect(height, width) => {
            return height*width;
        }
    };
    area
}

struct Rect {
    width : u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    // static function
    fn static_fn() -> bool{
        return true;
    }
}

fn get_length (str:String) -> usize {
    str.chars().count()
}

// find the fibbonacci series
// 0 1 1 2 3 5 8

fn fibbo(ind:u32) -> u32{
    if ind == 0 || ind == 1 {
        return ind;
    }

    let mut x: u32 = 0;
    let mut y: u32 =1;
    
    for _ in 1..ind{
        let res: u32 = x + y;
        x = y;
        y = res;
    }

    y
}

// function that takes a num an input and return is true or not

fn is_even(num:i32) -> bool {
    let res = num % 2 == 0;
    return res;
}

