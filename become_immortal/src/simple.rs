fn sum(row: usize, cols: usize, l: usize) -> usize {
    (0..cols as usize)
        .map(|i| {
            let r = i ^ row;
            if r > l {
                r - l
            } else {
                0
            }
        })
        .sum()
}

pub fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
    let (m, n) = match m > n {
        true => (m, n),
        false => (n, m),
    };

    let s: usize = (0..n as usize)
        .map(|r| sum(r, m as usize, l as usize))
        .sum();
    s as u64 % t
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
        // assert_eq!(
        //     elder_age(28827050410, 35165045587, 7109602, 13719506),
        //     5456283
        // );
        // assert_eq!(
        //     elder_age(3056346239594104477, 6886462835066995723, 621808, 1452456900),
        //     145780821
        // );
        // assert_eq!(elder_age(446121, 417857, 1056, 522447), 32813)
    }
}
