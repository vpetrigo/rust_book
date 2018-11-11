use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub struct Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + Hash + Clone,
          V: Clone
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + Hash + Clone,
          V: Clone
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: K) -> &V {
        match self.values.entry(arg.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(holder) => holder.insert((self.calculation)(arg)),
        }
    }
}

#[cfg(test)]
mod tests_cacher {
    use super::*;

    #[test]
    fn test_value_get() {
        let mut cacher = Cacher::new(|x| x);

        assert_eq!(*cacher.value(1), 1);
    }

    #[test]
    fn test_value_multiple_get() {
        let mut cacher = Cacher::new(|x| x);

        assert_eq!(*cacher.value(1), 1);
        assert_eq!(*cacher.value(2), 2);
    }

    #[test]
    fn test_single_call() {
        use std::cell::Cell;

        let init = Cell::new(false);

        let mut cacher = Cacher::new(|x: u32| {
            if !init.get() {
                init.set(true);
                x
            }
            else {
                panic!("Double call");
            }
        });

        assert_eq!(*cacher.value(1), 1);
        assert_eq!(*cacher.value(1), 1);
    }

    #[test]
    fn test_string_value_get() {
        let mut cacher = Cacher::new(|x: &str| x.len());

        assert_eq!(*cacher.value("123"), "123".len());
    }
}
