use std::fmt::Debug;

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

    fn debug_dump(&self) {
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
            cells: vec![HashCell::<_, _>::default(); self.cells.len() * 2],
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

fn main() {
    let mut phone_book = HashMap::<String, String>::new();
    for i in 0..11 {
        phone_book.insert(format!("{}", i), format!("{}", 1_000_000 + i));
    }
    phone_book.debug_dump();

    println!("--------------");
    phone_book.insert(format!("{}", 12), format!("{}", 1_000_000 + 12));
    phone_book.debug_dump();

    println!("--------------");
    for i in 0..11 {
        let key = format!("{}", i);
        let value = phone_book.get(&key).unwrap();
        println!("{} -> {}", key, value);
    }

    //     phone_book.insert("med fahmy".to_string(), "0652610981".to_string());
    //     phone_book.insert("john doe".to_string(), "7510000".to_string());
    //     phone_book.debug_dump();
    //
    //     let error_msg = String::from("no such key");
    //     let not_key = String::from("med fa");
    //     let key = String::from("med fahmy");
    //     let john_key = String::from("john doe");
    //
    //     println!("{}", phone_book.get(&not_key).unwrap_or_else(|| &error_msg));
    //     println!("{}", phone_book.get(&key).unwrap_or_else(|| &error_msg));
    //
    //     println!("{:?}", phone_book.get(&not_key));
    //     println!("{:?}", phone_book.get(&key));
    //     println!("{:?}", phone_book.get(&john_key));
}
