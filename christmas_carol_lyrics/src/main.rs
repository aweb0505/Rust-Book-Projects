fn main() {
    //A simple program that prints out the lyrics to "A Christmas Carol" taking advantage of the fact that the song is recursive.
    let mut day = 1;

    while day <= 12 {
        //Add st nd rd or th to the day
        if day == 1 {
            println!("On the {}st day of Christmas", day);
        } else if day == 2 {
            println!("On the {}nd day of Christmas", day);
        } else if day == 3 {
            println!("On the {}rd day of Christmas", day);
        } else {
            println!("On the {}th day of Christmas", day);
        }
        println!("My true love gave to me");

        let mut gift = day;

        while gift > 0 {
            match gift {
                1 => println!("A partridge in a pear tree"),
                2 => println!("Two turtle doves"),
                3 => println!("Three french hens"),
                4 => println!("Four calling birds"),
                5 => println!("Five golden rings"),
                6 => println!("Six geese a-laying"),
                7 => println!("Seven swans a-swimming"),
                8 => println!("Eight maids a-milking"),
                9 => println!("Nine ladies dancing"),
                10 => println!("Ten lords a-leaping"),
                11 => println!("Eleven pipers piping"),
                12 => println!("Twelve drummers drumming"),
                _ => println!("Error"),
            }
            gift -= 1;
        }
        day += 1;
    }
}
