fn floyd<T: Eq>(x0: &T, map: &dyn Fn(&T) -> &T) -> (u64, u64) {
    // Floyd's cycle detection algorithm.
    // Returns tuple (cycle period, cycle start)
   
    let mut tortoise = map(x0);
    let mut hare = map(map(x0));
    while tortoise != hare {
        tortoise = map(tortoise);  // tortoise moves by one step
        hare = map(map(hare));  // hare moves with double the speed
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
        tortoise = map(tortoise);
        hare = map(tortoise);  // Same speed
        mu += 1;
    }

    // The tortoise and hare are now both x(`mu`)
    // The cycle length `lambda` can be found advancing from x(`mu`) until match
    let mut lambda = 1;
    hare = map(tortoise);
    while tortoise != hare {
        hare = map(hare);
        lambda += 1;
    }

    return (lambda, mu)
}

fn crt_sieving(n_vec: &Vec<u64>, a_vec: &Vec<u64>) -> u64 {
    // The Chinese Remainder Theorem
    // Solves the relations x = a_i mod n_i for all i
    // `n` and `a` must have same lengths
    // n_i's must be coprime

    // Alg. more efficient when nis are in decreasing order
    let mut eqs: Vec<(u64, u64)> = std::iter::zip(n_vec, a_vec).map(|(n, a)| (*n, *a)).collect();
    eqs.sort_by_key(|(n, _)| Reverse(*n)); 
    
    let k = eqs.len();
    let (mut modulus, mut x) = eqs[0];
    for i in 0..k-1 {
        let (n, a) = eqs[i+1];
        while x % n != a {
            x += modulus;
        }
        modulus *= n
    }
    return x
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a%b);
    }
    return a
}

fn bezout(a: i64, b: i64) -> (i64, i64, i64) {
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
    };

    return (r.0, s.0, t.0)
}

fn crt_bezout(n_vec: &Vec<i64>, a_vec: &Vec<i64>) -> i64 {
    // The Chinese Remainder Theorem
    // Solves the relations x = a_i mod n_i for all i
    // `n` and `a` must have same lengths
    // n_i's must be coprime

    let mut eqs: Vec<(i64, i64)> = std::iter::zip(n_vec, a_vec).map(|(n, a)| (*n as i64, *a as i64)).collect();
    // eqs.sort_by_key(|(n, _)| Reverse(*n)); // do not know if sorting makes a difference
    
    let (mut n0, mut a0) = (1, 0);
    for (n1, a1) in eqs {
        let (_, m0, m1) = bezout(n0, n1);
        let x = a1*m0*n0 + a0*m1*n1;

        n0 = n0*n1;
        a0 = x % n0;
    };

    return if a0 > 0 {a0} else {a0 + n0};
}