use std::{
    io::{stdin, BufRead},
    ops::Add,
};

/// A wrapping iterator for ergonomic API that works like a moving sum over values of the inner iterator
struct MovingSumIterator<I, T> {
    inner: I,
    window_size: usize,
    window_buff: Vec<T>,
}

/// Extends all iterators that operate on items that can be copied & added together
impl<I, T> Iterator for MovingSumIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Add<Output = T> + Copy + Default,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.window_buff.len() < self.window_size {
            let next = self.inner.next()?;
            self.window_buff.push(next);
        }

        let sum: Self::Item = self.window_buff.iter().fold(T::default(), |a, b| a + *b);
        self.window_buff.drain(0..1);
        return Some(sum);
    }
}

/// Extension trait for IntoIterator
trait MovingSum<I, T>
where
    I: IntoIterator,
{
    fn moving_sum(self, window_size: usize) -> MovingSumIterator<I::IntoIter, T>;
}

impl<I, T> MovingSum<I, T> for I
where
    I: IntoIterator,
{
    fn moving_sum(self, window_size: usize) -> MovingSumIterator<I::IntoIter, T> {
        MovingSumIterator {
            inner: self.into_iter(),
            window_size,
            window_buff: Vec::with_capacity(window_size),
        }
    }
}

fn main() {
    let values: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|v| v.unwrap().parse::<u32>().unwrap())
        .collect();

    for window_size in [1, 3] {
        let mut inc_count = 0;
        let values: Vec<_> = values.clone().moving_sum(window_size).collect();

        for i in 1..values.len() {
            if values[i] > values[i - 1] {
                inc_count += 1;
            }
        }

        println!("Window size: {}, Increases: {}", window_size, inc_count);
    }
}
