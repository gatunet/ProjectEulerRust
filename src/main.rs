fn main() {

    let result: Vec<i64> = prime_factors(13195);
    println!("Result {:?}", result);

    let result: Vec<i64> = prime_factors(600851475143);
    println!("Result {:?}", result);

}

fn prime_factors(mut number: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = vec![];
    let mut divisor: i64 = 2;

    while number > 1 {

        while number % divisor == 0 {
            factors.push(divisor);
            number /= divisor;
        }

        divisor += 1;

        if divisor * divisor > number {
            if number > 1 {
                factors.push(number)
            }
            break
        }
    }

    return factors;
}
