use super::*;
use std::fmt::Debug;

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
where
    V: Eq,
    V: Debug,
    F1: Fn(&T, &T) -> V,
    F2: Fn(&usize, &usize) -> V,
{
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
                    let card1 = Card {
                        suit: SUITS[k],
                        value: VALUES[i],
                    };
                    let card2 = Card {
                        suit: SUITS[l],
                        value: VALUES[j],
                    };
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

fn equal_contents<T: PartialEq>(s1: &[T], s2: &[T]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    for i in s1 {
        if !s2.contains(i) {
            return false;
        }
    }

    return true;
}

#[test]
fn deck_from() {
    assert_eq!(Deck::from([]), Deck::<i32, 0> { cards: [], top: 0 });
    assert_eq!(
        Deck::from([1, 2, 3, 4, 5]),
        Deck {
            cards: [1, 2, 3, 4, 5],
            top: 0
        }
    );
}

#[test]
fn standard_deck() {
    let deck = standard();
    let mut expected = vec![];
    for suit in SUITS {
        for value in VALUES {
            expected.push(Card { suit, value })
        }
    }
    assert!(equal_contents(&deck.cards, &expected));
}

#[test]
fn get_one() {
    let deck = Deck::from([1, 2, 3]);
    assert_eq!(deck.get_one(), Some(&1));

    let deck = Deck {
        cards: [1, 2, 3],
        top: 1,
    };
    assert_eq!(deck.get_one(), Some(&2));

    let deck = Deck {
        cards: [1, 2, 3],
        top: 3,
    };
    assert_eq!(deck.get_one(), None);

    let deck: Deck<i32, 0> = Deck::from([]);
    assert_eq!(deck.get_one(), None);
}

#[test]
fn get_many() {
    let deck = Deck::from([1, 2, 3]);
    assert_eq!(deck.get_many(0), Vec::new() as Vec<&i32>);
    assert_eq!(deck.get_many(1), vec![&1]);
    assert_eq!(deck.get_many(2), vec![&1, &2]);
    assert_eq!(deck.get_many(3), vec![&1, &2, &3]);
    assert_eq!(deck.get_many(4), vec![&1, &2, &3]);

    let deck = Deck {
        cards: [1, 2, 3],
        top: 1,
    };

    assert_eq!(deck.get_many(0), Vec::new() as Vec<&i32>);
    assert_eq!(deck.get_many(1), vec![&2]);
    assert_eq!(deck.get_many(2), vec![&2, &3]);
    assert_eq!(deck.get_many(3), vec![&2, &3]);

    let deck = Deck {
        cards: [1, 2, 3],
        top: 3,
    };
    assert_eq!(deck.get_many(1), Vec::new() as Vec<&i32>);

    let deck: Deck<i32, 0> = Deck::from([]);
    assert_eq!(deck.get_many(1), Vec::new() as Vec<&i32>);
}

#[test]
fn get_all() {
    let deck = Deck::from([1, 2, 3]);
    assert_eq!(deck.get_all(), vec![&1, &2, &3]);

    let deck = Deck {
        cards: [1, 2, 3],
        top: 1,
    };
    assert_eq!(deck.get_all(), vec![&2, &3]);

    let deck = Deck {
        cards: [1, 2, 3],
        top: 3,
    };
    assert_eq!(deck.get_all(), Vec::new() as Vec<&i32>);

    let deck: Deck<i32, 0> = Deck::from([]);
    assert_eq!(deck.get_all(), Vec::new() as Vec<&i32>);
}

#[test]
fn draw_one() {
    let mut deck = Deck::from([1, 2, 3]);
    assert_eq!(deck.draw_one(), Some(&1));
    assert_eq!(deck.draw_one(), Some(&2));
    assert_eq!(deck.draw_one(), Some(&3));
    assert_eq!(deck.draw_one(), None);

    let mut deck: Deck<i32, 0> = Deck::from([]);
    assert_eq!(deck.draw_one(), None);
    assert_eq!(deck.draw_one(), None);
}

#[test]
fn draw_many() {
    let mut deck = Deck::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(deck.draw_many(0), Vec::new() as Vec<&i32>);
    assert_eq!(deck.draw_many(1), vec![&1]);
    assert_eq!(deck.draw_many(1), vec![&2]);
    assert_eq!(deck.draw_many(0), Vec::new() as Vec<&i32>);
    assert_eq!(deck.draw_many(3), vec![&3, &4, &5]);
    assert_eq!(deck.draw_many(10), vec![&6, &7, &8, &9, &10]);
    assert_eq!(deck.draw_many(10), Vec::new() as Vec<&i32>);
}

#[test]
fn shuffle() {
    // A large deck so there is a 1 in ~10^68 chance of shuffling to the starting position
    let orig = standard();
    let mut deck = orig.clone();
    deck.shuffle();
    for _ in 0..10 {
        assert_ne!(orig, deck);
        assert_eq!(orig.top, deck.top);
    }

    let orig = Deck {
        cards: orig.cards,
        top: 20,
    };
    let mut deck = orig.clone();

    for _ in 0..10 {
        deck.shuffle();

        assert_eq!(orig.cards[..20], deck.cards[..20]);
        assert_ne!(orig.cards[20..], deck.cards[20..]);
        assert_eq!(orig.top, deck.top);
    }

    let orig = Deck {
        cards: orig.cards,
        top: 52,
    };
    let mut deck = orig.clone();
    deck.shuffle();
    assert_eq!(orig, deck)
}

#[test]
fn reset() {
    let orig = standard();
    let mut deck = orig.clone();
    let _ = deck.draw_many(20);
    assert_ne!(deck, orig);
    deck.reset();
    assert_eq!(deck, orig);
}
