const MIN : i32 = 0;
const MAX : i32 = 1000;

// 1. Multiples of 3 and 5
fn main1() {
    fn sum_divisible_by(n : i32) -> i32 {
        let mut out = 0;
        let mut i = MIN;
        while i < MAX {
            out = out + i;
            i = i + n;
        }

        return out;
    }

    let result =
        sum_divisible_by(3) +
        sum_divisible_by(5) -
        sum_divisible_by(15);

    println!("1. {}", result == 233168);
}

// 2. Even Fibonacci numbers
fn main2() {
    fn even_fibbonaccis_below_a(limit : i32) -> i32 {
        let mut prev_fib = 1;
        let mut fib = 1;
        let mut sum = 0;
        while fib < limit {
            if fib % 2 == 0 {
                sum += fib;
            }
            let tmp = fib;
            fib += prev_fib;
            prev_fib = tmp;

        }
        return sum;
    }

    fn even_fibbonaccis_below_b(limit : i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        let mut c = a + b;
        let mut sum = 0;
        while c < limit {
            sum += c;
            a = b + c;
            b = c + a;
            c = a + b;
        }
        return sum;
    }

    let limit = 6000000;
    let result_a = even_fibbonaccis_below_a(limit);
    let result_b = even_fibbonaccis_below_b(limit);

    let solution = 4613732;

    println!("2. {} {}", result_a == solution, result_b == solution);
}


fn main() {
    main1();
    main2();
}
