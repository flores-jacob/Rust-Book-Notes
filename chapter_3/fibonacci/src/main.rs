use std::io;

fn main() {
    println!("Enter n for the fibonacci sequence");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().unwrap();

//    let n: u32 = match n.trim().parse() {
//        Ok(num) => num,
//        Err(_) => continue,
//    };
    let mut prevoius2_num: u32 = 0;
    let mut previous_num: u32 = 1;
    let mut current_num: u32 = 0;

    if n > 0 {
        for _ in 0..n -2 {
            current_num = previous_num + prevoius2_num;
            prevoius2_num = previous_num;
            previous_num = current_num;
        }
        println!("The fibonacci number at position {} is {}", n, current_num);
    } else {
        println!("The fibonacci number at position 1 is 0");
    }
}
