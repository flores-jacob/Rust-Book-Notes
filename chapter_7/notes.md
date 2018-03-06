# Chapter 7 Using Modules to Reuse and Organize Code

Definition: A module is a namespace that contains definitions of 
functions or types, and you can choose whether those definitions are 
visible outside their module (public) or not (private).

- The `mod` keyword declares a new module.
- By default, functions, types, constants, and modules are private. The 
`pub` keyword makes an item public and therefore visible outside its 
namespace.
- The `use` keyword brings modules, or the definitions inside modules, 
into scope so it’s easier to refer to them.

## 7.1 `mod` and the Filesystem

#### Creating a library
- By default, Cargo will create a library unless another type of project
is specified: if we omit the `--bin` option that we’ve been using in all 
of the chapters preceding this one, our project will be a library
    - Example:
        ```rust
        $ cargo new communicator
        ```
- Libraries cannot be compiled with `cargo run` since it has no `src/main.rs`
file
        
### Module definitions

- Example: Define a module named `network` that contains the definition of 
a function called `connect`. File is `src/lib.rs`
    ```rust
    mod network {
        fn connect() {
        }
    }
    ```
    - to call `connect` from outside the network module, we use 
    `network::connect()` rather than just `connect()`
    
- Example2: Define a module named `client` that also contains a `connect`
function
    ```rust
    mod network {
        fn connect() {
        }
    }
    
    mod client {
        fn connect() {
        }
    }    
    ```
- Example3: Define a module within a module in `src/lib.rs`
    ```rust
    mod network {
        fn connect() {
        }
    
        mod client {
            fn connect() {
            }
        }
    }    
    ```
    
### Moving Modules to Other Files

- Example: How to call an external module
    - `src/lib.rs`
    ```rust
    mod client;   // This is how we import from client.rs
                  // Basically, we end the module name with a semicolon
                  // instead of a block
    
    mod network {
        fn connect() {
        }
    
        mod server {
            fn connect() {
            }
        }
    }   
    ```
- Example2: How to define the external module 
    - `src/client.rs`
    ```rust
    fn connect() {
    }
    ```
    
- How to structure submodules
    1. Create a directory with the name of the main module
    2. Create a `mod.rs` file within this directory.  This file will
    contain the code for the module. Call ths submodules here using the
    `mod submodulename;` syntax
    3. Create `submodulename.rs` files within the directory which will
    contain the submodule code


### Rules of Module Filesystems

- If a module named foo has no submodules, you should put the declarations 
for foo in a file named foo.rs.
- If a module named foo does have submodules, you should put the declarations 
for foo in a file named foo/mod.rs.


## 7.2 Controlling Visibility with `pub`
- Rust modules and functions are private by default, and cannot be called
by external code.

### Making a Function Public
- To tell Rust to make a function public, we add the `pub` keyword to the 
start of the declaration.
- Example: To make the module `client` public, we use the code below:
    ```rust
    pub mod client;
    ```
- Example2: TO make the function `connect` public, we use the code below:
    ```rust
    pub fn connect() {
    }    
    ```
    
### Privacy Rules
1. If an item is public, it can be accessed through any of its parent modules.
2. If an item is private, it can be accessed only by its immediate parent 
module and any of the parent’s child modules

## 7.3 Referring to Names in Different Modules
- Example:
    ```rust
        pub mod a {
            pub mod series {
                pub mod of {
                    pub fn nested_modules() {}
                }
            }
        }
    
        use a::series::of; // Use the "use" keyword to bring "of" into scope
        
        fn main() {
            of::nested_modules(); // We can call of without mentioning all
                                  // of its parents
                                  // Otherwise, calling the same code will
                                  // be like so:
                                  // a::series::of::nested_modules();
        }    
    ```
    
- Example2: Bring `enums` variants into scope
    - if bringing multiple items from one namespace into scope, we
    can list them using curly brackets and commas in the last position, 
    like so:
    ```rust
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    use TrafficLight::{Red, Yellow};  // Note how both Red and Yellow
                                      // were brought into scope
    
    fn main() {
        let red = Red;
        let yellow = Yellow;
        let green = TrafficLight::Green;
    }
    ```
    
#### Bringing All Names into Scope with a Glob
- To bring all the items in a namespace into scope at once, we can use 
the * syntax, which is called the glob operator.
- Example:
    ```rust
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    use TrafficLight::*;  // Note how we use the * operator to bring all
                          // the enums variants into scope
    
    fn main() {
        let red = Red;
        let yellow = Yellow;
        let green = Green;
    }    
    ```     
    
#### Using `super` to Access a Parent Module
- Paths are always relative to the current module. The only exception is 
in a use statement, where paths are relative to the crate root by default.
- To refer to functions and modules contained in parent modules, there
are two options:
    1. Use leading colons to let Rust know that we want to start from 
    the root and list the whole path
        ```rust
        ::client::connect();
        ```
    2. Use `super` to move up one module in the hierarchy from our current 
    module
        ```rust
        super::client::connect();
        ```