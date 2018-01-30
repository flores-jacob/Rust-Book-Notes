# Chapter 5 Using Structs to Structure Related Data

## 5.1 Defining and Instantiating Structs
### Concepts
1. Explanatin of what a Struct is
    - Structs are similar to tuples. Like tuples, the pieces of a struct 
    can be different types. Unlike tuples, we name each piece of data so 
    it’s clear what the values mean.
    - Personal note: Maybe they're like named tuples in python?
    
    - Define a struct Example:
        ```rust
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }  
        ```
    - Instantiate a struct Example:
        ```rust
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };        
        ```
    - Access a specific value from struct example:
        ```rust
        user1.email
        ```
    - Change a field value of a mutable struct example:
        ```rust
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        
        user1.email = String::from("anotheremail@example.com");        
        ```
        - for this to work, entire struct has to be mutable
2. Struct intialization shorthand
    - If the parameter names of a function are identtical to the fieldnames
    of a struct, then we can also create an instance of the struct in this
    manner:
        ```rust
        fn build_user(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 1,
            }
        }   
        ```
    - If we are initializing a struct based on another instance, we can
    define it in a shorthand manner.
    - Usual way to define:
        ```rust
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            active: user1.active,
            sign_in_count: user1.sign_in_count,
        };        
        ```
    - Shorthand definition (note how `..user1` is used)
        ```rust
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };        
        ```
3. Tuple structs
    - These are basically tuples with names without fieldnames
    - Example:
        ```rust
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);        
        ```