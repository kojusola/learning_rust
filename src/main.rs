use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

fn main() {
//     println!("Guess the number!");
//     let secret_number = rand::thread_rng().gen_range(1..101);

//    loop{
//     println!("Please input your guess.");

//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     // let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//     println!("You guessed: {}", guess);

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => {
//             println!("You win!");
//             break;
//         },
//     }
//    }
    // const x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }

    // println!("The value of x is: {}", x);
    let  spaces = "  ";
    let spaces = spaces.len();
    println!("The value of x is: {}", spaces);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let six_point_four = tup.0;

    println!("The value of y is: {}", y);
    println!("The value of a is: {}", six_point_four);
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];
    // let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}