#![feature(test)]

use std::fmt::Debug;
use std::cmp::PartialEq;
use rand::distributions::{Distribution, Uniform};


trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut result: usize = 5381;
        for c in self.bytes() {
            result = ((result << 5).wrapping_add(result)).wrapping_add(c.into());
        }
        result
    }
}

impl Hashable for usize {
    fn hash(&self) -> usize {
        // let mut result: usize = 5381;
        // result = ((result << 5).wrapping_add(result)).wrapping_add(*self);
        // result
        *self
    }
}

#[derive(Default, Clone, Debug)]
struct HashCell<Key, Value> {
    key: Key,
    value: Value,
    taken: bool,
}

// impl<Key, Value> HashCell<Key, Value> {
//     unsafe fn as_mut(&self) -> &mut Self {
//         // &mut *self.as_ptr()
//         todo!()
//     }
//
//     fn as_ptr() {
//         todo!()
//     }
// }

#[derive(Debug)]
struct HashMap<Key, Value> {
    cells: Vec<HashCell<Key, Value>>,
    taken_count: usize,
}

impl<Key, Value> HashMap<Key, Value>
where
    Key: Clone + Default + Debug + PartialEq + Hashable,
    Value: Clone + Default + Debug,
{
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 11;
        Self {
            cells: vec![HashCell::<_, _>::default(); INITIAL_CAPACITY],
            taken_count: 0,
        }
    }

    fn debug(&self) {
        // &self.cells.into_iter().map(|cell| -> String {
        //     if cell.taken {
        //         format!("{:?} -> {:?}", cell.key, cell.value)
        //     } else {
        //         format!("x")
        //     }
        // } ).collect::<String>()

        for cell in &self.cells {
            if cell.taken {
                println!("{:?} -> {:?}", cell.key, cell.value);
            } else {
                println!("x");
            }
        }
    }

    fn insert(&mut self, key: Key, value: Value) {
        if let Some(old_value) = self.get_mut(&key) {
            *old_value = value;
        } else {
            if self.taken_count >= self.cells.len() {
                self.extend();
            }

            assert!(self.taken_count < self.cells.len());

            let mut index = key.hash() % self.cells.len();

            while self.cells[index].taken {
                index = (index + 1) % self.cells.len();
            }

            self.cells[index].taken = true;
            self.cells[index].key = key;
            self.cells[index].value = value;
            self.taken_count += 1;
        }
    }

    fn extend(&mut self) {
        assert!(self.cells.len() != 0);
        let mut new_self = Self {
            cells: vec![HashCell::<_, _>::default(); self.cells.len() * 2 + 1],
            taken_count: 0,
        };

        for cell in self.cells.iter() {
            if cell.taken {
                new_self.insert(cell.key.clone(), cell.value.clone());
            }
        }

        *self = new_self;
    }

    fn get_index(&self, key: &Key) -> Option<usize> {
        let mut index = key.hash() % self.cells.len();

        for _ in 0..self.cells.len() {
            if !self.cells[index].taken || self.cells[index].key == *key {
                break;
            }
            index = (index + 1) % self.cells.len();
        }

        if self.cells[index].taken && self.cells[index].key == *key {
            Some(index)
        } else {
            None
        }
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        // self.get_index(key).map(|index| &self.cells[index].value)

        match self.get_index(key) {
            Some(index) => Some(&self.cells[index].value),
            None => None,
        }
    }

    fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        // self.get_index(key).map(|index| &mut self.cells[index].value)

        match self.get_index(key) {
            Some(index) => Some(&mut self.cells[index].value),
            None => None,
        }

        // if let Some(index) = self.get_index(key) {
        //     Some(&mut self.cells[index].value)
        // } else {
        //     None
        // }
    }
}

fn load_hash_map() {
    let mut map = HashMap::<usize, usize>::new();
    for _ in 0..1000 {
        let key = rand::random::<usize>();
        if let Some(value) = map.get_mut(&key) {
            *value += 1;
        } else {
            map.insert(key, 1);
        }
    }
}

fn main() {
    let mut map = HashMap::<usize, usize>::new();
    for _ in 0..1000 {
        let key = rand::random::<usize>();
        if let Some(value) = map.get_mut(&key) {
            *value += 1;
        } else {
            map.insert(key, 1);
        }
    }
    map.debug();
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("map", |b| b.iter(|| load_hash_map()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
