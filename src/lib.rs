extern crate threadpool;

use threadpool::ThreadPool;

#[cfg(test)]
mod tests{
    #[test]
    fn it_works(){
        assert_eq!(2 + 2, 4);
    }
}

// Components needed for this:
// - Threadpool: Find existing
// - Regex pattern matcher: Find existing
// - Merp: job manager (Finds files in system and assigns to matchers): Write own
// - Matcher: Iterates through a file and finds all lines where "pattern" occurs

struct Merp{
    pool: ThreadPool,
}

