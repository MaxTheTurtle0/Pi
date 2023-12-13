use rand::Rng;

fn main() {
    let num_simulations = 1000000;
    let mut out_of_circle = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..num_simulations {
        let (x, y): (f64, f64) = (rng.gen(), rng.gen());
        if (x * x + y * y).sqrt() <= 1.0 {
            out_of_circle += 1;
        }
    }
    let pi = 4.0 * (out_of_circle as f64 / num_simulations as f64);
    println!("Pi: {:.2}", pi);
}
