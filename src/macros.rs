#[macro_export]
macro_rules! pool {
    () => {
        POOL.get().unwrap()
    };
}