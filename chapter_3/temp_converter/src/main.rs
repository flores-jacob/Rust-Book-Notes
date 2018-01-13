use std::io;

fn in_celcius(temp_in_f: f64) -> f64 {
    (temp_in_f - 32.0) * 5.0 / 9.0
}

fn in_farenheit(temp_in_c: f64) -> f64 {
    (temp_in_c * 9.0 / 5.0) + 32.0
}

fn main() {

    println!("Convert to what unit? (Enter C or F)");

    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Failed to read line");

    println!("Enter the temperature you want to convert");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f64 = temp.trim().parse().unwrap();

    match unit.trim(){
        "F" => println!("{} C is {} in F", temp, in_farenheit(temp)),
        "C" => println!("{} F is {} in C", temp, in_celcius(temp)),
         _  =>  println!("That is not a valid input")
    }
}
