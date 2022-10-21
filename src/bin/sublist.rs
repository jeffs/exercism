#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(xs: &[T], ys: &[T]) -> bool {
    ys.is_empty() || (ys.len() <= xs.len() && xs.windows(ys.len()).any(|w| w == ys))
}

pub fn sublist<T: PartialEq>(xs: &[T], ys: &[T]) -> Comparison {
    if xs == ys {
        Comparison::Equal
    } else if contains(ys, xs) {
        Comparison::Sublist
    } else if contains(xs, ys) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn main() {
    let jenny = [8, 6, 7, 5, 3, 0, 9];
    assert_eq!(sublist(&jenny, &jenny), Comparison::Equal);
    assert_eq!(sublist(&[7, 5, 3], &jenny), Comparison::Sublist);
    assert_eq!(sublist(&jenny, &[7, 5, 3]), Comparison::Superlist);
    assert_eq!(sublist(&jenny, &[42]), Comparison::Unequal);
}
