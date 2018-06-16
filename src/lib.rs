extern crate threadpool;
extern crate regex;

use threadpool::ThreadPool;
use regex::Regex;

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
    query: Regex,
    files: Regex,
}

struct MerpBuilder {
    workers: usize,
    query: Option<String>,
    files: Option<String>,
}

impl Merp{
    fn new() -> MerpBuilder {
        MerpBuilder::new()
    }
}

impl MerpBuilder{
    fn new() -> Self {
        return Self { workers: 1, query: None, files: None }
    }

    fn workers(&mut self, w: usize) {
        self.workers = w;
    }

    fn query(&mut self, q: String) {
        self.query = Some(q);
    }

    fn files(&mut self, f: String) {
        self.files = Some(f);
    }

    fn build(self) -> Merp {
        let q = self.query.unwrap_or(r".*".to_string());
        let f = self.files.unwrap_or(r"\./.*".to_string());
        let query = Regex::new(&q).expect("Failed to compile query regexp");
        let files = Regex::new(&f).expect("Failed to compile files regexp");
        let pool = ThreadPool::new(self.workers);
        return Merp {pool, query, files}
    }
}

