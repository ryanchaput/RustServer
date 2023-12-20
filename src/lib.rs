pub struct ThreadPool;

impl ThreadPool {

    pub fn new(size: u32) -> ThreadPool {
        //Panic if size is not positive
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}