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
        