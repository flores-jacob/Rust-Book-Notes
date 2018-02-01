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
        
## 5.2 An Example Program Using Structs
### Refactor a rectangle program to use structs

1. Original code
    ```rust
    fn main() {
        let width1 = 30;
        let height1 = 50;
    
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }
    
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }    
    ```
    
2. Use tuples
    ```rust
    fn main() {
        let rect1 = (30, 50);
    
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
    }
    
    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }  
 
    ```
    
3. Add more meaning - Creat a Rectangle struct to clarigy intentions
    ```rust
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
    
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
    }
    
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }  
    ```
4. Add useful functionality with Derived Traits - Enable printing rect
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
    
        println!("rect1 is {:?}", rect1);
    }    
    ```
    
## 5.3 Method Syntax

### Concepts
1. What are methods?
    - Methods are similar to functions: they’re declared with the fn 
    keyword and their name, they can have parameters and a return value, 
    and they contain some code that is run when they’re called from 
    somewhere else.
2. So are they basically functions?
    - They are different from functions, in that they are always defined
    within a struct, enum, or trait object, and their first parameter
    is always `self` which represents the instance of the struct the
    method is being called on
3. Example code - define an `area` method on the `Rectangle` struct
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
    
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }  
    ```
    - Note how "methods" are implemented within an `impl` block. The 
    explanation given by the book is to group all things we can do with
    an instance type in a single location.
    - "methods" can be called later on with dot notation. Examples:
        ```rust
        rect1.area()
        ```
4. Another example: create a can_hold method for rectangles
    ```rust
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
 
    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };
    
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    } 
    ```        
        
5. Associated functions
    - These are functions in the `impl` block that do not take `self` as a
    parameter
    - Probably similar to class functions in python
    - Can be used as constructors
    - Example:
        ```rust
        impl Rectangle {
            fn square(size: u32) -> Rectangle {
                Rectangle { width: size, height: size }
            }
        }        
        ```
6. It is possible to have multiple impl blocks for each struct. Example:
    ```rust
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }    
    ```
    - Not sure what the benefit is yet though, since this feels 
    antithetical to the previously declared benefit of keeping things
    in one place