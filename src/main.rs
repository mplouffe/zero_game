use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Destroy Blocks!");

    let block_distance = rand::thread_rng().gen_range(20..101);

    println!("The block is: {}m away from you.", block_distance);

    // game loop
    loop {
        // read and parse angle
        println!("Input angle.");

        let mut angle = String::new();

        io::stdin()
            .read_line(&mut angle)
            .expect("Failed to read line.");

        let angle: f64 = angle.trim().parse().expect("Please type a number!");

        println!("Angle set: {}°", angle);
        //

        // read and parse power
        println!("Input power.");

        let mut power = String::new();

        io::stdin()
            .read_line(&mut power)
            .expect("Failed to read line.");
        
        let power: f64 = power.trim().parse().expect("Please type a number!");

        println!("Power set: {}", power);
        //

        // calculate shot result
        let shot_result = (2.0*(power.powf(2.0))*angle.sin()*angle.cos())/9.8;

        let shot_result = shot_result as i32;

        println!("Your shot travelled {}m!", shot_result);
        //

        // compare
        match shot_result.cmp(&block_distance) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Kaboom!");
                
                break; // exit from loop
            }
        }
    }
    // game loop
}
