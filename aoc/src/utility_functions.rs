pub fn floyd<T: Clone + Eq>(x0: T, map: &dyn Fn(&T) -> T) -> (usize, usize) {
    // Floyd's cycle detection algorithm.
    // Returns tuple (cycle period, cycle start)

    let mut tortoise = map(&x0);
    let mut hare = map(&map(&x0));
    while tortoise != hare {
        tortoise = map(&tortoise); // tortoise moves by one step
        hare = map(&map(&hare)); // hare moves with double the speed
    }

    // If the position (number of steps taken) of the tortoise is `nu` then the position of the hare will be 2 * `nu`
    // Since they are the same: x(`nu`) = x(2 * `nu`), the position `nu` must be a multiple of the period `lambda`

    // Let `mu` be the starting position of the cycle
    // Since `nu` = n * `lambda` we know that x(`mu`) = x(`mu` + 2*`nu`)
    // This can be found by advancing `mu` steps from x(0) and x(2*`nu`) simultaneously

    // Find the starting position of the cycle
    let mut mu = 0;
    tortoise = x0;
    while tortoise != hare {
        tortoise = map(&tortoise);
        hare = map(&tortoise); // Same speed
        mu += 1;
    }

    // The tortoise and hare are now both x(`mu`)
    // The cycle length `lambda` can be found advancing from x(`mu`) until match
    let mut lambda = 1;
    hare = map(&tortoise);
    while tortoise != hare {
        hare = map(&hare);
        lambda += 1;
    }

    return (lambda, mu);
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Euclidean algorithm for determining the
    // Greatest Commen Divisor (GCD) of `a` and `b`
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

pub fn lcm(a: u64, b: u64) -> u64 {
    // Least Common Multiple (LCM) of `a` and `b`
    return (a / gcd(a, b)) * b;
}

pub fn bezout(a: i64, b: i64) -> (i64, i64, i64) {
    // Extended Euclidean algorithm for determining
    // Returns (gcd, s, t) such that Bezout's identity holds
    // gcd(a, b) = a*s + b*t

    let mut r = (a, b);
    let mut s = (1_i64, 0_i64);
    let mut t = (0_i64, 1_i64);
    let mut q;

    while r.1 != 0 {
        q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }

    return (r.0, s.0, t.0);
}

pub fn crt_solve(n_vec: &Vec<i64>, a_vec: &Vec<i64>) -> i64 {
    // The Chinese Remainder Theorem (CRT)
    // Solves the relations x = a_i mod n_i for all i
    // `n` and `a` must have same lengths
    // n_i's must be coprime

    let eqs: Vec<(i64, i64)> = std::iter::zip(n_vec, a_vec)
        .map(|(n, a)| (*n as i64, *a as i64))
        .collect();
    // eqs.sort_by_key(|(n, _)| Reverse(*n)); // do not know if sorting makes a difference

    let (mut n0, mut a0) = (1, 0);
    for (n1, a1) in eqs {
        let (_, m0, m1) = bezout(n0, n1);
        let x = a1 * m0 * n0 + a0 * m1 * n1;

        n0 = n0 * n1;
        a0 = x % n0;
    }

    return if a0 >= 0 { a0 } else { a0 + n0 };
}

pub fn extrapolate(sequence: &Vec<i64>, n: usize) -> i64 {
    // Returns the n'th number in the sequence by polynomial extrapolation
    // using Newton's forward difference formula. It is assumed that the
    // values are taken equidistantly => val[i] = f(ih) for some stepsize h.
    // https://en.wikipedia.org/wiki/Newton_polynomial.

    let mut coefficients = vec![sequence[0]];
    let mut line = sequence.clone();
    while !line.iter().all(|x| x == &line[0]) {
        let diff: Vec<i64> = line[1..]
            .iter()
            .enumerate()
            .map(|(i, x)| x - line[i])
            .collect();
        coefficients.push(diff[0]);
        line = diff;
    }

    // Degree of the polynomial that fits sequence.
    let degree = coefficients.len();
    // The extrapolated value
    let result = coefficients
        .iter()
        .zip(binoms(n, degree))
        .map(|(a, b)| a * b as i64)
        .sum();

    return result;
}

fn binoms(n: usize, k: usize) -> Vec<usize> {
    // All binomial coefficients (n \\ i) for 0 <= i <= k
    let mut result = vec![1];
    for i in 0..k {
        result.push(result[i] * (n - i) / (i + 1));
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(81, 231), 6237);
        assert_eq!(lcm(6237, 3465), 31185);
    }

    #[test]
    fn test_bezout() {
        let (a, b) = (177741, 149553);
        let (gcd_, s, t) = bezout(a, b);

        assert_eq!(gcd_, gcd(177741, 149553) as i64);
        assert_eq!(gcd_, a * s + b * t);
    }

    #[test]
    fn test_floyd() {
        let fun = |x: &i32| ((x + 2) % 64) as i32;

        assert_eq!(floyd(1, &fun), (32, 0));
    }

    #[test]
    fn test_crt() {
        let n_vec = vec![3, 4, 5];
        let a_vec = vec![0, 3, 4];
        assert_eq!(crt_solve(&n_vec, &a_vec), 39);
    }

    #[test]
    fn test_extrapolate() {
        let sequence = vec![3, 6, 18, 72];
        let steps = vec![4, 5, 6, 7, 8];

        let true_vals = vec![201, 438, 816, 1368, 2127];
        let extrapolated = steps
            .into_iter()
            .map(|n| extrapolate(&sequence, n))
            .collect::<Vec<_>>();

        assert_eq!(true_vals, extrapolated);
    }
}
