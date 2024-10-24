use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{0}:{1}", count, v)?;
        }

        write!(f, "]")
    }
}

fn printer() {
    let v = List(vec![1, 2, 3]);

    println!("{}", v);

    println!(
        "This is the first element: {0} and second: {1} and first again: {0}",
        "1st", "2nd"
    );

    println!(
        "{one} is to err, {two} is to run with {three}",
        one = "Something",
        two = "Everything",
        three = "Nothing"
    );

    println!("{number:0>5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Pi is roughly `{pi:.3}`");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {:#?} won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
}

fn binary_search(arr: &[u32], num: u32, min: usize, max: usize) -> usize {
    let index: usize = (min + max) / 2;

    if arr[index] == num {
        return index;
    } else if index == 0 {
        return 0;
    } else if arr[index] > num {
        let new_max = index - 1;

        binary_search(arr, num, min, new_max)
    } else {
        let new_min = index + 1;

        binary_search(arr, num, new_min, max)
    }
}

fn primitives() {
    let tuple_1 = (43u8, (50i32, false), true, 4.3f32, 2555i16, "hey", 'i');

    println!(
        "this is the first element in the tuple element in the tuple: {}",
        tuple_1.1 .0
    );

    println!("this is the tuple in the tuple: {:?}", tuple_1.1);

    let (a, b) = tuple_1.1;

    println!("{:?} & {:?}", a, b);

    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));
}

fn main() {
    primitives();
    let primes: [u32; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    let max1 = primes.len();

    let result = binary_search(&primes, 67, 0, max1);

    println!("found number in {result}");

    printer();
}
