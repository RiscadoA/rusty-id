pub trait Id: Clone + Copy {
    fn from_index(index: usize) -> Self;
    fn to_index(self) -> usize;
}
