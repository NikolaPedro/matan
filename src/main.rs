use matan::Sum;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let epsilon: f64 = input.trim().parse().unwrap();

    let sum = Sum::new(1, 20, function).get_value(epsilon);

    println!("{}", sum);
}

fn function(n: i64) -> f64 {
    let n: f64 = n as f64;
    (n + 1f64) / (3f64.powf(n) + n + 1f64)
}

fn function_a(n: i64) -> f64 {
    let fact = factorial(n) as f64;
    let n: f64 = n as f64;
    (2.0 + n.sqrt()) / fact
}

fn factorial(value: i64) -> i64 {
    (1..=value).product()
}
