

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
//use rand::Rng;

fn main() {
    println!("Hello, world! 11");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret - {}", secret_number);


    println!("input your guess number");
    let mut guess = String::new();    

    io::stdin().read_line(&mut guess)
    .expect("failed");


    let guess : u32 = guess.trim().parse()
    .expect("please type a bumber");

    println!("you guess {}", guess);

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("too small!"),
    //     Ordering::Greater => println!("big"),
    //     Ordering::Equal => { 
    //         println!("you win !!!!!!!!");
    //         break;
    //     }
    // }


    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => {
            println!("You win!");
            //break;
        }
    }

}

//cargo add rand






