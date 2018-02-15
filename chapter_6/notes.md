# Chapter 6 Enums and Pattern Matching

## 6.1 Defining an enum

### Concepts:

1. What is an `enum`?
    - Enumerations, also referred to as enums, allow you to define a type by enumerating its possible values. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. These are the only possibilities for an IP address that our program will come across: we can enumerate all possible values, which is where enumeration gets its name.
2. _variants_ of an enum - enumeration and listing of the different kinds included in the enum.
    - Example:  Defining an `IpAddrKind` enumeration and listing the possible kinds an IP address can be, `V4` and `V6`.
        ```rust
        enum IpAddrKind {
            V4,
            V6,
        }    
        ```
        - Create instances of the above enum
        - Example:
            ```rust
            let four = IpAddrKind::V4;
            let six = IpAddrKind::V6;        
            ```
        - Write a function
            ```rust
            fn route(ip_type: IpAddrKind) { }            
            ```
        - Call the function with either variant
            ```rust
            route(IpAddrKind::V4);
            route(IpAddrKind::V6);            
            ```
        - Define a struct using a defined enum
            ```rust
            enum IpAddrKind {
                V4,
                V6,
            }
            
            struct IpAddr {
                kind: IpAddrKind,
                address: String,
            }
            
            let home = IpAddr {
                kind: IpAddrKind::V4,
                address: String::from("127.0.0.1"),
            };
            
            let loopback = IpAddr {
                kind: IpAddrKind::V6,
                address: String::from("::1"),
            };            
            ```
        - Associate data with the enums
            ```rust
            enum IpAddr {
                V4(String),
                V6(String),
            }
            
            let home = IpAddr::V4(String::from("127.0.0.1"));
            
            let loopback = IpAddr::V6(String::from("::1"));            
            ```
        - Associate variable types and amounts of data with each enum
            ```rust
            enum IpAddr {
                V4(u8, u8, u8, u8),
                V6(String),
            }
            
            let home = IpAddr::V4(127, 0, 0, 1);
            
            let loopback = IpAddr::V6(String::from("::1"));            
            ```
3. Any kind of data can be put inside an enum variant (strings, numeric 
types, or structs, another enum etc)
    - Example:
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    } 
    
    // struct equivalent:
 
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct    
    ```
4. It is possible to define methods for enums in the same way it is possible
to define methods for structs using `impl`
    - Example:
        ```rust
        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }
        
        let m = Message::Write(String::from("hello"));
        m.call();    
        ```
5. The `Option` enum
    - Basically implements the NULL concept from other programming languages
        ```rust
        enum Option<T> {
            Some(T),
            None,
        }
        ```
    - No need to bring these into scope explicitly
        ```rust
        let some_number = Some(5);
        let some_string = Some("a string");
        
        let absent_number: Option<i32> = None;        
        ```
        
## 6.2 The `match` Control Flow Operator

### Concepts

1. `match` - is a control flow operator that  allows us to compare a value 
against a series of patterns and then execute code based on which pattern 
matches.
    - Patterns can be made up of literal values, variable names, wildcards. etc
    - Probably works similarly to `switch` statements from other languages 
    - Example:
        ```rust
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        
        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }    
        ```
2. `match` arms
    - An arm has two parts: a pattern and some code. The first 
    arm here has a pattern that is the value Coin::Penny and then 
    the => operator that separates the pattern and the code to run. 
    The code in this case is just the value 1. Each arm is separated 
    from the next with a comma.
            
    - Patterns that Bind to Values
        - Another useful feature of match arms is that they can bind to 
        parts of the values that match the pattern.
        - Probably similar to capturing values in regexes?
        ```rust
        #[derive(Debug)] // So we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
            // ... etc
        }
        
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
  
        //usage
        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                },
            }
        }
  
        ``` 
        
    - Matching with `Option<T>`
        - Example:
        ```rust
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);        
        ```
        
3. Matches are exhaustive
    - All possible cases need to be handled otherwise the code won't compile
    
4. The `_` Placeholder
    - Used when we do not want to list all values
    - `_` matches any value
    - Example:
        ```rust
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }        
        ```
## 6.3 Concise Control Flow with `if let`
1. The if let syntax lets you combine if and let into a less verbose way 
to handle values that match one pattern and ignore the rest.
    - Example:
        ```rust
        // Usual matching, too much boilerplate for one pattern    
    
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
        ```
        ```rust
        // If let syntax
        if let Some(3) = some_u8_value {
            println!("three");
        }
        ``` 
        
    - Example2: Count all non-quarter coins we see while and announce the 
    state of the quarters,
        ```rust
        // match syntax
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
        ```
        ```rust
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }        
        ```