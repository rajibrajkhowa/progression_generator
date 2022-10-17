use std::env;
/// This is a simple RUST program to generate Arithmetic Progression, Geometrice Progression and Harmonic Progression
///
/// The usage is cargo run <initial value> <common difference/common ratio> <number of terms> <type of progression to be generated>
///
/// Say, if I want to generate an Arithmetic Progression with initial value 1, common difference 2 and say upto 10 terms,
/// I would write the command as cargo run 1 2 10 AP
///
/// Say, if I want to generate an Geometric Progression with initial value 1, common ratio 2 and say upto 10 terms,
/// I would write the command as cargo run 1 2 10 GP
///
/// Say, if I want to generate an Harmonic Progression with initial value 1, common difference 2 and say upto 10 terms,
/// I would write the command as cargo run 1 2 10 HP
use std::process::exit;

pub const MAX: f64 = f64::MAX;
pub const MIN: f64 = f64::MIN;

#[derive(Debug)]

struct Input {
    a: f64,
    d: f64,
    n: f64,
    s: String,
}

impl Input {
    fn ap_generator(&self) -> Vec<f64> {
        if *(&self.s) == "AP" {
            let mut output: Vec<f64> = Vec::new();
            let mut b = *(&self.a);
            let x = *(&self.n) as u64;
            output.push(b);

            for _i in 1..x {
                b = b + *(&self.d);
                if b < MAX {
                    output.push(b);
                } else {
                    exit(1);
                }
            }
            return output;
        } else {
            exit(1);
        }
    }

    fn gp_generator(&self) -> Vec<f64> {
        if *(&self.s) == "GP" {
            let mut output: Vec<f64> = Vec::new();
            let mut b = *(&self.a);
            let x = *(&self.n) as i32;
            output.push(b);

            for i in 1..x {
                b = *(&self.a) * (*(&self.d)).powi(i) as f64;
                if b < MAX {
                    output.push(b);
                } else {
                    exit(1);
                }
            }
            return output;
        } else {
            exit(1);
        }
    }
    fn hp_generator(&self) -> Vec<f64> {
        if *(&self.s) == "HP" {
            let mut output: Vec<f64> = Vec::new();
            let mut b = (*(&self.a)).powi(-1);
            let x = *(&self.n) as u64;
            output.push(b);

            for _i in 1..x {
                b = b + *(&self.d);
                let c: f64 = b.powi(-1);
                if c > MIN {
                    output.push(c);
                } else {
                    exit(1);
                }
            }
            return output;
        } else {
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let w = &args[1];
    let w: f64 = w.trim().parse().expect("Please type a number");

    if w == 0.0 {
        println!("The first element of a progression cannot be zero");
        exit(1);
    }

    let x = &args[2];
    let x: f64 = x.trim().parse().expect("Please type a number");

    let y = &args[3];
    let y: f64 = y.trim().parse().expect("Please type a number");

    let z = &args[4];

    let input = Input {
        a: w,
        d: x,
        n: y,
        s: z.to_string(),
    };

    match input.s.trim() {
        "AP" => println!(
            "The arithmetic progression generated for {:?} is \n {:#?}",
            input,
            input.ap_generator()
        ),
        "GP" => println!(
            "The geometric progression generated for {:?} is \n {:#?}",
            input,
            input.gp_generator()
        ),
        "HP" => println!(
            "The harmonic progression generated for {:?} is \n {:#?}",
            input,
            input.hp_generator()
        ),
        _ => println!("Something went wrong. No progression type entered. Try again"),
    }
}
