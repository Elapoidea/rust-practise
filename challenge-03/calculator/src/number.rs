pub fn prime_factors(upto: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = vec![];

    'main: for i in 2..=upto {
        if upto % i != 0 {
            continue
        }

        for j in &primes {
            if i % j == 0 {
                continue 'main;
            }
        }

        primes.push(i)
    }

    primes.insert(0, 1);

    primes
}

pub fn common_factors(x: (i64, i64)) -> Vec<i64> {
    let mut factors = vec![];

    for i in prime_factors(x.0) {
        for j in prime_factors(x.1) {
            if i == j { factors.push(i) }
        }
    }

    factors
}

pub fn gcf(x: (i64, i64)) -> i64 {
    *common_factors(x).last().unwrap()
}

pub fn lcm(x: (i64, i64)) -> i64 {
    x.0 * x.1 / gcf(x)
}

pub fn factorial(x: (i64, i64)) -> i64 {
    if x.0 >= 2 && x.0 >= x.1 {
        x.0 * factorial((x.0 - 1, x.1))
    } else {
        1
    }
}
