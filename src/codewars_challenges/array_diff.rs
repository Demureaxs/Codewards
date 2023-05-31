use std::fmt::Debug;

        pub fn run_array_diff() {
            array_diff(vec![1, 2], vec![1]);
            array_diff_b(vec![1, 2], vec![1]);
        }

        fn array_diff<T: PartialEq + Clone + Debug>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
            let mut result: Vec<T> = Vec::new();

            for num in a {
                if !b.contains(&num) {
                    result.push(num);
                }
            }
            println!("{:?}", result);
            result
        }

        fn array_diff_b<T: PartialEq + Debug>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
            a.retain(|num| !b.contains(num));
            println!("{:?}", a);
            a
        }