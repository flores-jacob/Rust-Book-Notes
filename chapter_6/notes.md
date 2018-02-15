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
