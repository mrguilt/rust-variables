fn main() {
    println!("Hello. Moving on to Chapter 3, Section 1: Variables and Mutability\n");   //Just checking: escape sequences ("\n") still work.

    println!("This is the original listing, where x is first assigned 5, then assigned 6");
    //Listing 3-1-1
    //let x = 5; //original
    let mut x=5; //Updated (Listing 3-1-2)
    println!("The value of x is: {x}");
    x = 6; //FAIL! ^^^^^ cannot assign twice to immutable variable
    println!("The value of x is: {x}\n");

    println!("This is my listing, where y is first assigned 5, then adding 1. y will not be mutable on the first go--and it failed. Adding mut keyword on second pass.");
    //Listing 3-1-1, Charles's version
    //let y = 5; //original
    let mut y=5; //second pass, with mut keyword
    println!("The value of y is: {y}");
    y = y+2; //Fail--this counts as a second assginment
    println!("The value of y is: {y}");
    println!("So, variables, by default, are \"really\" constants.")


}
