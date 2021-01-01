mod simple;

fn pow_2(n: u64) -> u64 {
    let mut ret = 1;
    while ret < n {
        ret <<= 1;
    }
    ret
}

fn range_sum(l: u64, r: u64, t: u64) -> u64 {
    let (a, b) = (l + r, r + 1 - l);
    let (a, b) = match a & 1 == 0 {
        true => (a / 2 % t, b % t),
        false => (a % t, b / 2 % t),
    };
    (a * b) % t
}

fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
    if m == 0 || n == 0 || (m == 1 && n == 1) {
        return 0;
    }

    let (m, n) = match m >= n {
        true => (m, n),
        false => (n, m),
    };

    let pm = pow_2(m);
    let pn = pow_2(n);

    if l >= pm {
        return 0;
    }

    if pm == pn {
        return (range_sum(1, pm - l - 1, t) * ((m + n - pm) % t)
            + elder_age(pm - n, pm - m, l, t))
            % t;
    }

    let pn = pm / 2;
    let mut age = range_sum(1, pm - l - 1, t) * (n % t);
    match l <= pn {
        true => {
            let a = (pn - n) % t;
            let b = (pm - m) % t;
            let c = (pn - l) % t;
            age += ((a * b % t) * c % t + elder_age(pm - m, pn - n, 0, t)) % t;
            let d = ((pm - m) % t) * range_sum(pn - l, pm - l - 1, t) % t;
            // println!("{}, {}", age, d);
            age = age + t - d;
        }
        false => {
            age += elder_age(pm - m, pn - n, l - pn, t);
            age -= ((pm - m) % t) * range_sum(0, pm - l - 1, t) % t;
        }
    };
    age % t
}

fn main() {
    println!("{}", simple::elder_age(509, 501, 12, 6321));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(elder_age(8, 5, 1, 100), 5);
        assert_eq!(elder_age(8, 8, 0, 100007), 224);
        assert_eq!(elder_age(25, 31, 0, 100007), 11925);
        assert_eq!(elder_age(5, 45, 3, 1000007), 4323);
        assert_eq!(elder_age(31, 39, 7, 2345), 1586);
        assert_eq!(elder_age(545, 435, 342, 1000007), 808451);
        assert_eq!(
            elder_age(28827050410, 35165045587, 7109602, 13719506),
            5456283
        );
        assert_eq!(
            elder_age(3056346239594104477, 6886462835066995723, 621808, 1452456900),
            145780821
        );
        assert_eq!(elder_age(446121, 417857, 1056, 522447), 32813)
    }

    #[test]
    fn simple_tests() {
        assert_eq!(
            elder_age(509, 501, 12, 6321),
            simple::elder_age(509, 501, 12, 6321)
        )
    }
}
