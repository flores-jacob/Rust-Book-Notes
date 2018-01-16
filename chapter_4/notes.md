# Chapter 4 Ownership

## 4.1 What is ownership?
### Concepts:
1. Stack and Heap
    1. Stack
        - Last in first out
        - Like a stack of plates
        - _Push_ - to add an item onto the top of a stack
        - _Pop_ - to take the topmost item from a stack
        - Fast, since it only has to add and take items from the top
        - However, all the data in the stack must take up a known, fixed, 
        pre determined size
    2. Heap
        - We use this primarily when we don't know at the outset what the
        size of the data we will be putting in will be.  Or, maybe we know,
        but we don't know what size it will be if it changes.
        - This is slower than the stack system
        - When we have data, we feed it to the computer, which then first
        looks for a place with as much empty memory that we require.  It
        saves the data there, then returns a _pointer_ to us, which tells
        us where it put it.
        - This is much like having a bunch or _heap_ of stuff in one place.
        You put stuff there, and even if you know where you put the stuff,
        it'll take time for you to find it and bring it out.
        
2. Some data can be mutated, and some cannot
    - Example:
        - String literals `let s = "hello";`
            - canNOT be mutated
            - mutable because we know what it's size is at the outset, and
            it is hardcoded into the binary at compile time
        - String types `let s = String::from("hello");`
            - CAN be mutated
            - because it is allocated on the _heap_ at runtime, not compile
            time
            - Ultimately we do this allocation at runtime because we do
            not know at compile time what amount of memory it will occupy
            - In other programming languages, allocated memory that is no
            longer in use is _Garbage Collected_
            - In rust, when a variable goes out of scope, the variable gets
            deallocated
3. Variables with different data act differently even with the same actions
    - Example:
        - integers vs String types
            - Integers (and other variables in the _stack_)
                ```rust
                let x = 5;
                let y = x;
                ```
                - The `y` will make a copy of `x`.  Modifying `y` will not affect `x`.
                - The behavior such, because the sizes of the variables are known
                beforehand, and they are pushed onto the _stack_, not the _heap_
                - calling `x` even after this reassignment will not result 
                in a compile time error.  This is because integers are 
                basically copy types. These are types that basically have
                fixed memory, do not need to be allocated, and that live
                in a stack
            - String types (not string _literals_) (and other objects in 
            the _heap_)
                ```rust
                let s1 = String::from("hello");
                let s2 = s1;
                ```
                - A `s2` does NOT copy `s1` in this instance
                - `s2` copies the pointer to the location of the data that
                `s1` has instead
                - Calling `s1` again after _moving_ it's data to `s2` will
                cause a move error at compile time.  This is because the
                data is now associated with `s2` and not `s1`
                - Problem - How do we get a copy of the data and leave the
                original untouched?
                    - Solution 1 - perform a `clone`.
                        - Example:
                        ```rust
                        let s1 = String::from("hello");
                        let s2 = s1.clone();
                        
                        println!("s1 = {}, s2 = {}", s1, s2);
                        ```
                        - downside: cloning is a fairly expensive operation
            - Function parameters
                - If a variable is of `copy` type, once passed onto a function,
                it can still be used afterwards.
                - If a variable is of `move` type, once pased onto a function,
                it can no longer be used afterwards, and will return a compile
                time error if done so.
            - Functions and return values
                - Functions can give back ownership of variables passed 
                into them as parameters in their `return` statements.
                - Downside - Requiring that all variables to be used as
                parameters be passed in, and returned can be very tedious
                
## 4.2 References and borrowing
1. References (or _borrowing)
    - Since constantly passing all variables to functions, and returning
    them as return values to preserve ownership can be extremely 
    cumbersome, we have references, which allow us to pass variables
    to functions without passing ownership, so that we can still use
    the function later on
    - Downsides:
        - _Borrowed_ variables need to be returned
        - _Borrowed_ variables cannot be modified
        - _Borrowed_ variables are _immutable_ by default
        - Example: this will work:
            ```rust
            fn calculate_length(s: &String) -> usize { // s is a reference to a String
                s.len()
            }
            let s1 = String::from("hello");
            let len = calculate_length(&s1); // calculate_length _borrows_ s1
        - Example: this will NOT work, it will return a compilation error:
            - This is because it attempts to modify a borrowed variable 
            ```rust
            fn main() {
                let s = String::from("hello");
            
                change(&s);
            }
            
            fn change(some_string: &String) {
                some_string.push_str(", world");
            }            
            ```
        
    - Question: So what if we need to modify borrowed variables passed as 
    parameters?
        - To accomplish this, we need to do two things:
            1. Make sure that the variable being passed in to be borrowed is
            _mutable_
            2. Make sure that the signature of the function is defined to 
            accept and borrow a _mutable_ reference.
    
        - Example:
            ```rust
            fn change(some_string: &mut String) { // Step 1: Ensure that the function is expecting a borrowed
                some_string.push_str(", world");  // variable, and that it's meant to be mutable
            }            
            fn main() {
                let mut s = String::from("hello"); // Step 2, make the variable to be borrowed mutable
                change(&mut s); // pass in the mutable variable
            }

            ```
        - Downside:
            - We can only have one reference to a piece of mutable data 
            for any given scope.
            - This is to prevent data races (similar to race conditions)
            - Example: this code will NOT compile:
                ```rust
                let mut s = String::from("hello");
                
                let r1 = &mut s;
                let r2 = &mut s;            
                ```
            - Solution is to use curly brackets to isolate each reference, 
            to avoid _simultaneous_ references
            - Example: This code will compile:
                ```rust
                let mut s = String::from("hello");
                {
                    let r1 = &mut s;
                } // r1 goes out of scope here, so we can make a new reference with no problems.
                let r2 = &mut s;            
                ``` 
            - We also cannot combine mutable and immutable references
            - Example: This code will NOT compile:
                ```rust
                let mut s = String::from("hello");
                
                let r1 = &s; // no problem
                let r2 = &s; // no problem
                let r3 = &mut s; // BIG PROBLEM                
                ```
                - Lesson: multiple borrows are OK as long as variable is
                borrowed as _immutable_, since we are just reading the data,
                and not writing to it
    - Dangling references
        - Rust prevents dangling references at compile time.
        - If we return any references without any borrower, then the 
        compilation fails
        - The soltuion is to return the variable directly and not a 
        reference to it
        - Example: This will NOT compile:
            ```rust
            fn dangle() -> &String { // dangle returns a reference to a String

                let s = String::from("hello"); // s is a new String
            
                &s // we return a reference to the String, s
            } // Here, s goes out of scope, and is dropped. Its memory goes away.
              // Danger!
            ```
        - Example: this WILL compile:
            ```rust
            fn no_dangle() -> String {
                let s = String::from("hello");
                s
            }
            ```
    - Main takeaways:
        1. At any given time, you can have either but not both of:
            - One mutable reference.
            - Any number of immutable references.
        2. References must always be valid.
## 4.3 Slices
1. The problem
    - You have a String type variable, and you get a reference to it's first 
    word/letter, etc.  Later on however, you change the value of the variable.
        - Question:
            - When you call the reference, does it return the value based
            on the original unchanged variable? Or does it return the value 
            of the new changed variable?
        - Answer:
            - It depends.  However, we do know that this is a potential source
            of multiple bugs in other languages
        - How Rust handles things:
            - Rust will still compile, however, the reference will no longer
            be valid if we do effect a change to the original variable
            - To go around this, we will need to use what Rust calls `slices`
            - Slices
                - `slices` are portions of a string (or an array)
                - when we create a slice, rust saves the values found in
                that portion of the string onto the variable
                - Even if we change the contents of the original variable,
                the slice remains the same and valid, same as the time the
                slice was taken
2. Slice notation - how to get a slice
    - Example
        ```rust
        let s = String::from("hello world");
        
        let hello = &s[0..5];
        let world = &s[6..11];                 
        ```
3. Slice notation - slice type
    - Example - function with signature that returns a slice
        ```rust
        fn first_word(s: &String) -> &str {    
        ```
    - Example - function with signature that takes AND returns a slice
        ```rust
        fn first_word(s: &str) -> &str {    
        ```
        - NOTE: When creating functions, it is better to accept parameters
        of type slice (`&str`) than of type String (`&String`)
        
                    
4. Arrays can also be sliced
    - Example
        ```rust
        let a = [1, 2, 3, 4, 5];
        
        let slice = &a[1..3];    
        ```