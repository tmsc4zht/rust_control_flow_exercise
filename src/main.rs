fn main() {
    let x = 0.0;
    println!("{} °C is {} °F", x, celsius_to_fahrenheit(x));
    println!("{} °F is {} °C", x, fahrenheit_to_celsius(x));

    let x = 100.0;
    println!("{} °C is {} °F", x, celsius_to_fahrenheit(x));
    println!("{} °F is {} °C", x, fahrenheit_to_celsius(x));

    let n = 10;
    println!("{}th Fibonacci number is {}", n, fibonacci_number(n));

    the_twelve_days_of_christmas();
}

fn celsius_to_fahrenheit(celcius: f32) -> f32 {
    9.0 / 5.0 * celcius + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    5.0 / 9.0 * (fahrenheit - 32.0)
}

fn fibonacci_number(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    let mut x: (u32, u32) = (1, 1);

    for i in 0..n - 1 {
        x = (x.1, x.0 + x.1);
        println!("{}:{}", i + 2, x.1)
    }

    return x.1;
}

fn the_twelve_days_of_christmas() {
    // https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics
    let ordinal_numeral = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let lyric_lines = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    for i in 0..12 {
        println!("{}.", i + 1);
        println!("On the {} day of Christmas,", ordinal_numeral[i]);
        println!("my true love sent to me");

        // n = 0 => from 11 to 11
        // n = 1 => from 10 to 11
        // n = 11 => from 0 to 11
        for lyric_line in lyric_lines[(11 - i)..].iter() {
            println!("{}", lyric_line);
        }
        println!("");
    }
}
