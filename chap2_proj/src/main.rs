use std::io::{self, Write};

fn main() {
    if false {
        func_f2c();
    }

    if false {
        func_fibonacci();
    }

    if false {
        func_the_twelve_days_of_christmas();
    }
}

fn func_f2c() {
    let mut f = Default::default();

    print!("input temperature in Fahrenheit: ");
    io::stdout().flush().expect("failed to flush stdout");

    io::stdin().read_line(&mut f).expect("failed to read line.");
    let f: f32 = f.trim().parse().expect("failed to parse number");

    let c = f2c(f);
    println!("Fahrenheit: {f}, => Celsius: {c}");
}

// Fahrenheit degree to Celsius degree
// c = (f - 32) * 5 / 9
fn f2c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn func_fibonacci() {
    let mut num = Default::default();

    print!("input the index in fibonacci array: ");
    io::stdout().flush().expect("failed to flush stdout");

    io::stdin()
        .read_line(&mut num)
        .expect("failed to read line.");
    let num: u32 = num.trim().parse().expect("failed to parse number");

    let num = fibonacci(num);
    println!("Fibonacci element: {num}");
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut feb: [u32; 2] = [0, 1];
    let mut ret: u32 = Default::default();
    for _ in 2..=n {
        ret = feb[0] + feb[1];
        feb[0] = feb[1];
        feb[1] = ret;
    }

    ret
}

fn func_the_twelve_days_of_christmas() {
    let things = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    let th = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..things.len() {
        println!("On the {} day of Christmas my true love sent to me", th[i]);

        for j in (0..=i).rev() {
            println!(
                "{}{}",
                if i != 0 && j == 0 { "and " } else { "" },
                things[j]
            );
        }
    }
}
