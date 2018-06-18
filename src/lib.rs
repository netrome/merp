extern crate threadpool;
extern crate regex;

use threadpool::ThreadPool;
use regex::Regex;
use std::path::Path;
use std::iter::Iterator;

use std::fs::{self, DirEntry, ReadDir};

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

pub struct Merp{
    pool: ThreadPool,
    query: Regex,
    files: Regex,
}

pub struct MerpBuilder {
    workers: usize,
    query: Option<String>,
    files: Option<String>,
}

impl Merp{
    pub fn new() -> MerpBuilder {
        MerpBuilder::new()
    }

    pub fn match_files(&self){
        let base_dir = String::from(Path::new("./").to_str().expect("sdfg"));
        let mut q = Vec::new();
        q.push(base_dir);
        /*
        // Search over directories
        while(!q.is_empty()){
            let s: String = q.pop().expect("Empty q");
            let index = Path::new(&s);
            if index.is_dir(){
                index
                    .read_dir().expect(&format!("Unable to read directory: {:?}", index.to_str()))
                    .for_each(|entry| q.push(String::from(entry.expect("asdfg").path().to_str().unwrap())));
            }
            println!("Currently at: {}", s);
        }*/
        let mut iter = FileIter{ q:q };
        for s in iter {
            println!("Visited: {}", s);
        }
    }
}

impl MerpBuilder{
    pub fn new() -> Self {
        return Self { workers: 1, query: None, files: None }
    }

    pub fn workers(mut self, w: usize) -> Self{
        self.workers = w; self
    }

    pub fn query(mut self, q: String) -> Self {
        self.query = Some(q); self
    }

    pub fn files(mut self, f: String) -> Self{
        self.files = Some(f); self
    }

    pub fn  build(self) -> Merp {
        let q = self.query.unwrap_or(r".*".to_string());
        let f = self.files.unwrap_or(r"\./.*".to_string());
        let query = Regex::new(&q).expect("Failed to compile query regexp");
        let files = Regex::new(&f).expect("Failed to compile files regexp");
        let pool = ThreadPool::new(self.workers);
        return Merp {pool, query, files}
    }
}

// TODO: Re-implement the search as an iterator
struct FileIter{q: Vec<String>}

impl Iterator for FileIter{
    type Item = String;
    fn next(&mut self) -> Option<String>{
        if let Some(s) = self.q.pop(){
            let mut c = false;
            {
                let f = Path::new(&s);
                c = f.is_file();
                if f.is_dir(){
                    f   .read_dir().expect(&format!("Unable to read directory: {:?}", f.to_str()))
                        .for_each(|entry| self.q.push(String::from(entry.expect("asd").path().to_str().unwrap())));
                    return self.next();
                }
            }
            if c{
                return Some(s);
            }
        }
        return None;
    }
}

