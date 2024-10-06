#[macro_use] extern crate rocket;

#[get("/calculate_prime_factors/<number>")]
fn process(number: i64) -> String {
    let result: Vec<i64> = prime_factors(number);
    return format!("Result {:?}", result);
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![process])
}
