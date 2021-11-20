use std::env;

use rand::{prelude::IteratorRandom, thread_rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let rng = &mut thread_rng();
        let mut numbers = (1..from+1).choose_multiple(rng, take);
        numbers.sort_unstable();

        Lotto { take, from, numbers }
    }

    // get_numbers used by test
    #[allow(dead_code)]
    fn get_numbers(self) -> Vec<usize> {
        self.numbers
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    return format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // ignore first arg as it is script name
    let args_len = args.len() - 1;

    if args_len < 2 {
        println!("program must have be at least 2 args");
        return
    }

    if args_len % 2 != 0 {
        println!("program args must be multiple of 2");
    } else {
        for i in (1..args_len).step_by(2) {
            let take: usize = args[i].parse().expect("Could not parse number");
            let from: usize = args[i+1].parse().expect("Could not parse number");

            if take > from {
                println!("take must not be lower than from: {} !< {}", take, from);
                return
            }

            let lotto = Lotto::new(take, from);

            println!("{}", format_lotto_results(&lotto));
        }
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
