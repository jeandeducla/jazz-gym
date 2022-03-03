use std::ops::Add;

pub trait Source: Iterator
where
    Self::Item: Add<Self::Item>,
{
}
