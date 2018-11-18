pub mod average_collections {
    pub struct IntAverageList {
        list: Vec<i64>,
        average: f64,
    }

    impl IntAverageList {
        pub fn new() -> IntAverageList {
            IntAverageList {
                list: vec![],
                average: 0.0
            }
        }

        pub fn add(&mut self, elem: i64) {
            self.list.push(elem);
            self.update_average();
        }

        pub fn remove(&mut self, index: usize) -> i64 {
            let prev_max = self.average * self.list.len() as f64;
            let remove_elem = self.list.remove(index);
            self.average = (prev_max - remove_elem as f64) / self.list.len() as f64;

            remove_elem
        }

        fn update_average(&mut self) {
            let prev_max = self.average * (self.list.len() - 1) as f64;
            let new_max = match self.list.last() {
                Some(&elem) => prev_max + elem as f64,
                None => prev_max
            };

            self.average = new_max / self.list.len() as f64;
        }

        pub fn get_average(&self) -> f64 {
            self.average
        }

        pub fn len(&self) -> usize {
            self.list.len()
        }
    }
}

#[cfg(test)]
mod average_list_test {
    use super::*;

    #[test]
    fn test_add() {
        let mut avg_list = average_collections::IntAverageList::new();

        avg_list.add(1);
        assert_eq!(1.0, avg_list.get_average());
        avg_list.add(2);
        assert_eq!(1.5, avg_list.get_average());
        avg_list.add(3);
        assert_eq!(2.0, avg_list.get_average());
    }

    #[test]
    fn test_remove() {
        let mut avg_list = average_collections::IntAverageList::new();

        for &i in &[1, 2, 3] {
            avg_list.add(i);
        }

        let cur_size = avg_list.len();
        avg_list.remove(cur_size - 1);
        assert_eq!(1.5, avg_list.get_average());
        let cur_size = avg_list.len();
        avg_list.remove(cur_size - 1);
        assert_eq!(1.0, avg_list.get_average());
    }
}