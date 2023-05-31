pub fn run_product_fib() {
    // product_fib(4895);
    product_fib(5895);
    product_fib_codewars_solution(5895);
}

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let result: (u64, u64, bool) = (0, 0, false);
    // base
    let mut a = 0;
    // current number
    let mut b = 1;
    // while b is less than the input value
    while b < prod {
        // create a temp variable assigned to the value of b
        let temp = b;
        // increment b by the value of a
        b += a;
        // set a to the value of temp
        a = temp;

        // check the multiples of a and b if its less than the input value
        if a * b < prod {
            continue;
        } else if a * b == prod {
            return (a, b, true);
        } else if a * b > prod {
            return (a, b, false);
        }
    }
    return result;
}

fn product_fib_codewars_solution(prod: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;
    while a * b < prod {
        let c = a + b;
        a = b;
        b = c;
    }
    (a, b, a * b == prod)
}