fn main() {

 let mut x=5;
 x=6;

print_number(x,x);

}
// Doesnt allow pattern matching over functions
fn print_number(x: i32) -> i32{
    println!("x is: {}", x);
}

/*fn print_number(x: i32, y: i32) {
    println!("x is: {}", x);
}/*