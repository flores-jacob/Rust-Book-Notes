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

## 6.1 `mod` and the Filesystem

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
