//Next section: Constants!
const SECONDS_IN_YEAR: u32=60*60*24*365; //Not a leap year, but I wasn't sure if this was a float or an integer

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
    println!("So, variables, by default, are \"really\" constants.\n");

    //Next section: Constants!
    println!("Well, not really.");
    println!("There are {SECONDS_IN_YEAR} seconds in a year.\n");

    //Shadowing!
    println!("Shadowing...");
    let x=x*2;
    println!("I did let x=x*2. I got {x}.");
    let z=5;
    let z=z*z;
    println!("the second pass of z is {z}");

    let spaces="    ";          //string variable
    println!("Putting spaces (string) between dots: .{spaces}.");
    let mut spaces=spaces.len();    //shadow variable of number type--a new variable
    println!("Initial space length is {spaces}.");
    spaces=spaces*2;
    println!("Space length doubled is {spaces}");
    let mut dots=".....";       //mutable sting varable
    //dots=dots.len();          //This would fail due to trying to assign a type number to a string (same variable)


}
