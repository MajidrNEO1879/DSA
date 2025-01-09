use std::io;
fn main() {
    let mut todo_list:Vec<(String)> = vec![];
    println!("please add your todos...");
    let mut todo_item:String = String::new();
    io::stdin().read_line(&mut todo_item).unwrap();   //just reading the input 
    let input:String = todo_item.trim().to_string(); 
   

}
