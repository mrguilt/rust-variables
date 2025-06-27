# Rust Variables
Playing with Rust variables, basically [Chapter 3, Section 1, "Variables and Mutability"](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).

*That, and continuing to learn GitHub.*

## Leasons I've Learned

* In Rust, variables are, by default, effectively constants. 
    * But wait! There *is* such a thing as a constant! WTF is the difference between an immutable variable and a constant? Perhaps I need to keep reading. 
    * OK, [googled a bit](https://www.reddit.com/r/rust/comments/pj2ier/the_main_difference_between_unmutable_variables/). The main difference is that a const must be known at compile time; a variable cannot. So a constant array must be set to 5 items, where I can set a variable array to a literally random number, different each run. 
    * Can a variable become mutable after initial declaration (or, conversely a mutable one cease to be so)?
* Shadow variables basically mean you're making a new variable with same name. It makes some sense if we're talking about two different types; less so for the same type.
    * How does Rust know which variable I'm using? It can grok it if they're different types. What if they are the same type?
    
