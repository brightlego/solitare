use std::fmt::Debug;
use super::*;

const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace
];

const SUITS: [Suit; 4] = [
    Suit::Clubs,
    Suit::Hearts,
    Suit::Diamonds,
    Suit::Spades
];

/// Tests whether a function applied to pairs of values in the slice return the same values
/// as another function applied on the pairs of indices.
///
/// # Arguments
///
/// * `values`: The slice of values to get pairs from
/// * `value_func`: The function to apply on the pairs of values
/// * `index_func`: The function to apply on the pairs of indices
///
/// returns: ()
///
/// # Examples
/// ```
/// // Tests if the array is sorted by Ord (without assuming Ord generates a valid ordering)
/// let arr = ["a", "b", "c", "d", "e"];
/// test_returns_same(&arr, |s1, s2| s1.cmp(s2), |i, j| i.cmp(j));
/// ```
/// ```
/// // Tests if the array contains distinct elements
/// let arr = [1, 2, 5, 10, -3];
/// test_returns_same(&arr, |i, j| i == j, |i, j| i == j);
/// ```

fn test_returns_same<T, V, F1, F2>(values: &[T], value_func: F1, index_func: F2)
        where V: Eq, V: Debug, F1: Fn(&T, &T) -> V, F2: Fn(&usize, &usize) -> V {
    for i in 0..values.len() {
        for j in 0..values.len() {
            assert_eq!(value_func(&values[i], &values[j]), index_func(&i, &j));
        }
    }
}

#[test]
fn suit_eq() {
    test_returns_same(&SUITS, Suit::eq, |i, j| i == j)
}

#[test]
fn value_eq() {
    test_returns_same(&VALUES, Value::eq, |i, j| i == j)
}

#[test]
fn value_ord() {
    test_returns_same(&VALUES, Value::cmp, |i, j| i.cmp(j))
}

#[test]
fn card_ord() {
    for i in 0..VALUES.len() {
        for j in 0..VALUES.len() {
            for k in 0..SUITS.len() {
                for l in 0..SUITS.len() {
                    let card1 = Card { suit: SUITS[k], value: VALUES[i]};
                    let card2 = Card { suit: SUITS[l], value: VALUES[j]};
                    let cmp = card1.partial_cmp(&card2);

                    if k == l {
                        assert_eq!(cmp, Some(i.cmp(&j)));
                    } else {
                        assert_eq!(cmp, None);
                    }
                }
            }
        }
    }
}