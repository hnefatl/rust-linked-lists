pub trait List<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self);
    fn front(&mut self) -> Option<&mut T>;

    //fn push_back(&mut self, item: T);
    //fn pop_back(&mut self);
    //fn back(&self) -> &mut T;

    fn len(&self) -> usize;

    // TODO(kwc): Replace with iterator impls
    fn to_vec(&mut self) -> Vec<&mut T>;
}