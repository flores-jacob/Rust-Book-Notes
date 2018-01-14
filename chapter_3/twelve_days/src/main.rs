fn main() {
    let verses = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves and",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming"
    ];

    let intro = "On the {} day of Christmas my true love sent to me:";

    let days = ["first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for i in 1..13 {
        println!("On the {} day of Christmas my true love sent to me:", days[i - 1]);
        for k in 1..i + 1 {
            println!("{}", verses[i - k]);
        }
        println!{"\n"}
    }
}
