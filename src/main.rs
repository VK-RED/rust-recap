use std::fs::read_to_string;
use chrono::Local;
fn main() {
    
    let current_time = Local::now();
    println!("Current time is : {}", current_time);
    
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

