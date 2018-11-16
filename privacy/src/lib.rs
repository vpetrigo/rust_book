pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
        }

        pub fn secret_function() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_me() {
        outermost::middle_function();
        //outermost::middle_secret_function();
        outermost::inside::inner_function();
        outermost::inside::secret_function();
    }
}