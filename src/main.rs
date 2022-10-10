use std::process::exit;
use std::io;

#[derive(Debug)]

struct Input {
    a: f64,
    d: f64,
    n: f64
}


trait ApGen {
    fn ap_generator(&self) -> Vec<f64>;
}


impl ApGen for Input {

    fn ap_generator(&self) -> Vec<f64> {

        let mut output: Vec<f64> = Vec::new();
        let mut b = *(&self.a);
        let x = *(&self.n) as u64;
        output.push(b);

        for _i in 1..x {
            b = b + *(&self.d);
            output.push(b);
        }
        return output;
    }
}


trait GpGen {
    fn gp_generator(&self) -> Vec<f64>;
}


impl GpGen for Input {

    fn gp_generator(&self) -> Vec<f64> {

        let mut output: Vec<f64> = Vec::new();
        let mut b = *(&self.a);
        let x = *(&self.n) as i32;
        output.push(b);

        for i in 1..x {
            b = *(&self.a) * (*(&self.d)).powi(i) as f64;
            output.push(b);
        }
        return output;
    }
}


trait HpGen {
    fn hp_generator(&self) -> Vec<f64>;
}


impl HpGen for Input {

    fn hp_generator(&self) -> Vec<f64> {

        let mut output: Vec<f64> = Vec::new();
        let mut b = (*(&self.a)).powi(-1);
        let x = *(&self.n) as u64;
        output.push(b);

        for _i in 1..x {
            b = (b + *(&self.d)).powi(-1);
            output.push(b);
        }
        return output;
        }
}



fn main() {

    let mut x = String::new();
    println!("Please enter the initial number:");
    io::stdin().read_line(&mut x).expect("Number not entered");
    let x: f64 = x.trim().parse().expect("Please type a number");

    if x == 0.0 {
        println!("The first element of a progression cannot be zero");
        exit(1);

    }

    let mut y = String::new();
    println!("Please enter the common difference or common ratio:");
    io::stdin().read_line(&mut y).expect("Number not entered");
    let y: f64 = y.trim().parse().expect("Please type a number");

    let mut z = String::new();
    println!("Please enter the number of terms to be generated:");
    io::stdin().read_line(&mut z).expect("Number not entered");
    let z: f64 = z.trim().parse().expect("Please type a number");

    let input = Input { a: x, d: y, n: z};
    let mut prog = String::new();
    println!("Please enter the type of progression to be generated [AP or GP or HP]:");
    io::stdin().read_line(&mut prog)
       .ok()
       .expect("Type of progression to be generated is not entered");
    
    match prog.trim() {

        "AP" =>  println!("The arithmetic progression generated for {:?} is {:?}", input, input.ap_generator()),
        "GP" =>  println!("The geometric progression generated for {:?} is {:?}", input, input.gp_generator()),
        "HP" =>  println!("The harmonic progression generated for {:?} is {:?}", input, input.hp_generator()),
         _ =>    println!("Something went wrong. No progression type entered. Try again")
    }

}
