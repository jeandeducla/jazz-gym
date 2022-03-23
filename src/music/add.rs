use rodio::{Sample, Source};

pub struct Add<T> {
    input1: T,
    input2: T,
}

impl<T> Iterator for Add<T>
where
    T: Source,
    T::Item: Sample,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match (&self.input2.next(), &self.input2.next()) {
            (Some(i1), Some(i2)) => Some(i1+i2),
    }
}
