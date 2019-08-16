mod list;
mod singlelinked;
extern crate rand;

#[cfg(test)]
extern crate strum;
#[cfg(test)]
#[macro_use]
extern crate strum_macros;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::list::*;
    use super::singlelinked;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    use strum::EnumCount;

    type ItemType = u8;

    #[derive(EnumCount)]
    enum TestAction {
        Push(ItemType),
        Pop,
        Front,
    }
    use TestAction::*;

    impl Distribution<TestAction> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TestAction {
            let num_variants = 3;
            // Ensure we're covering all possible actions
            assert_eq!(num_variants, TestAction::count());
            match rng.gen_range(0, num_variants) {
                0 => Push(rand::random()),
                1 => Pop,
                2 => Front,
                _ => panic!("Missed case in sample for TestAction"),
            }
        }
    }

    #[test]
    fn test_single_linked() {
        let get_list = || singlelinked::SingleLinked::new();
        test_list(get_list)
    }

    fn assert_vec_eq(left: Vec<&mut ItemType>, right: &Vec<ItemType>) {
        assert_eq!(left.len(), right.len());
        for (y, x) in right.iter().zip(left) {
            assert_eq!(x, y);
        }
    }
    fn test_list<F, L: List<ItemType>>(get_list: F)
    where
        F: Fn() -> L,
    {
        test_list_specific(get_list());
        test_list_random(get_list());
    }
    fn test_list_random<L: List<ItemType>>(mut tested: L) {
        let mut truth = vec![];
        for _ in 0..100 {
            match rand::random() {
                Push(x) => {
                    tested.push(x);
                    truth.insert(0, x);
                }
                Pop => {
                    tested.pop();
                    if !truth.is_empty() {
                        truth.remove(0);
                    }
                }
                Front => assert_eq!(tested.front(), truth.first_mut()),
            }
            println!("tested={:?}, truth={:?}", tested.to_vec(), truth);
            assert_vec_eq(tested.to_vec(), &truth);
        }
    }
    fn test_list_specific<L: List<ItemType>>(mut tested: L) {
        tested.push(2);
        assert_vec_eq(tested.to_vec(), &vec![2]);
        tested.push(1);
        assert_vec_eq(tested.to_vec(), &vec![1, 2]);
        tested.pop();
        assert_vec_eq(tested.to_vec(), &vec![2]);
        tested.push(1);
        assert_vec_eq(tested.to_vec(), &vec![1, 2]);
        tested.pop();
        tested.pop();
        assert_vec_eq(tested.to_vec(), &vec![]);
        tested.push(1);
        assert_vec_eq(tested.to_vec(), &vec![1]);
    }
}
