//Day 4,5th June 2024
//Shadowing
// You can declare a new variable with the same name as a previous variable here we can say ** the first variable is shadowed by the second one

//only modify `assert_eq!`  to make the `println! ` work(print `42` in terminnal)
 fn main(){
let x: i32 = 5;
{
 let x = 12;
 assert_eq!(x,5);
 
}
assert_eq(x,5);

let x = 42;
println!("{}", x); //Prints "42".
}
