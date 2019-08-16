mod list;
mod singlelinked;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::list::*;
    use super::singlelinked;

    type ItemType = i32;

    enum TestAction {
        Push(ItemType),
        Pop,
        Front,
    }

    fn test_list<T, L: List<T>>(l: &mut L){
        
    }

    #[test]
    fn test_single_linked() {
        let mut l = singlelinked::SingleLinked::<u32>::new();
        test_list(&mut l)
    }
}