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
    for i in 0..12 {
        println!("{}.", i + 1);
        n_day_of_christmas(i);
        println!("");
    }
}

fn n_day_of_christmas(n: usize) {
    // https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics
    let ordinal_numeral = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let presents = &presents[0..n + 1];

    println!("On the {} day of Christmas,", ordinal_numeral[n]);
    println!("my true love sent to me");

    if presents.len() >= 3 {
        for idx in (2..presents.len()).rev() {
            println!("{}", presents[idx])
        }
    }
    if presents.len() >= 2 {
        println!("{}, and", presents[1]);
    }
    println!("{}", presents[0]);
}
