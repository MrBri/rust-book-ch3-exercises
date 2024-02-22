fn fib_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fib_iter(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
 
#[cfg(test)]
mod tests {
    use super::fib_recursive;
    use super::fib_iter;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);

        assert_eq!(fib_iter(0), 0);
        assert_eq!(fib_iter(1), 1);
    }

    #[test]
    fn test_fibonacci_first_few() {
        assert_eq!(fib_recursive(2), 1);
        assert_eq!(fib_recursive(3), 2);
        assert_eq!(fib_recursive(4), 3);
        assert_eq!(fib_recursive(5), 5);
        assert_eq!(fib_recursive(10), 55);
        assert_eq!(fib_recursive(37), 24157817);

        assert_eq!(fib_iter(2), 1);
        assert_eq!(fib_iter(3), 2);
        assert_eq!(fib_iter(4), 3);
        assert_eq!(fib_iter(5), 5);
        assert_eq!(fib_iter(10), 55);
        assert_eq!(fib_iter(37), 24157817);
    }
}

