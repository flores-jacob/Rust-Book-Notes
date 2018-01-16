# Rust Book Notes and sample code

### What is this?
These are my notes from reading the Rust Book (2nd edition), including some
sample code, as well as all exercise code.

### Anything of interest?
So far, these are the areas of note:

#### Runnable exercises

- [How to run the exercises](How-to-run-the-exercises)

1. Temperature converter mini program written in rust [view the code](./chapter_3/temp_converter/src/main.rs)
    - This was the first recommended exercise in chapter 3
2. Fibonacci program that returns nth fib number written in rust [view the code](./chapter_3/fibonacci/src/main.rs)
    - This was the second recommended exercise in chapter 3
3. Twelve days of Christmas lyrics written in rust [view the code](./chapter_3/twelve_days/src/main.rs)
    - This was the third recommended exercise in chapter 3
    
#### Notes
1. [Notes on Ownership, borrowing, and references](./chapter_4/notes.md)
    - Some personal notes to help retain material I'm reading.  Not proofread
    or organized for that matter. May contain errors, but reflects my
    understanding of the material at the moment of writing


### Appendices

#### How to run the exercises
1. Install [rust](https://www.rust-lang.org/en-US/install.html)
2. Run `cargo new some_name --bin`. This will create a folder named
`some_name` which has the proper directory structure to hold the script 
that we will be downloading next.
    ```
    $ cargo new some_name --bin
    ```
3. Either download one of the sample codes [above](####-Runnable-exercises)
and overwrite the `main.rs` file found in the previously created directory located at:
    ```bash
    some_name/src/main.rs
    ```
    or copy paste the file contents into the aforementioned `main.rs` file.
4. Change into the directory of the `some_name` folder created earlier
    ```bash
    $ cd ./some_name
    ```
5. Run `cargo run`. This should compile the downloaded code and download
dependencies, as well as run the program after compilation
    ```build
    $ cargo run
    ```
    