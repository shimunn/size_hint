use size_hint::HintSize;
use std::iter::successors;

fn main() {
    let iter = successors(Some(0), |prev| match prev {
        counter if *counter < 10 => Some(counter + 1),
        _ => None,
    })
    .hint_size(10);
    println!("Iterator size: {}", iter.size_hint().0);
    iter.for_each(|n| println!("{}", n));
}
