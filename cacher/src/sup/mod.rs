use std::collections::HashMap;

pub struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests_cacher {
    use super::*;

    #[test]
    fn test_value_get() {
        let mut cacher = Cacher::new(|x| x);

        assert_eq!(cacher.value(1), 1);
    }

    #[test]
    fn test_value_multiple_get() {
        let mut cacher = Cacher::new(|x| x);

        assert_eq!(cacher.value(1), 1);
        assert_eq!(cacher.value(2), 2);
    }

    #[test]
    fn test_single_call() {
        use std::cell::Cell;

        let init = Cell::new(false);

        let mut cacher = Cacher::new(|x| {
            if !init.get() {
                init.set(true);
                x
            }
            else {
                panic!("Double call");
            }
        });

        assert_eq!(cacher.value(1), 1);
        assert_eq!(cacher.value(1), 1);
    }
}
