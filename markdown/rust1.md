# Variables

Binding and Mutability
1. a variable can be used only if it has been initialised
// Fix the error below withn atleast amount of modification to the code
fn main() {
let x: i32 = 5; // Uninitialized but used , ERROR!
let _y: i32; // Uninitialised but also unused, only a Warning!

assert_eq!(x,5);
println!("Success!")
}
