
fn main() {
    let v = vec![1_u8, 2, 3];
    v.each(|item| println!("step: {}", item));
}

/// A trait to iterate through collections.
pub trait Each<T> {
    /// For each value in a collection returns reference to a value
    /// calling `f` function.
    fn each(&self, f: |&T|);
}

impl<T> Each<T> for Vec<T> {
    fn each(&self, f: |&T|) {
        for v in self.iter() {
            f(v);
        }
    }
}
