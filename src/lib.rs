extern crate threadpool;
extern crate regex;

use threadpool::ThreadPool;
use regex::Regex;
use std::path::Path;
use std::iter::Iterator;
use std::sync::mpsc::{channel, Sender};
use std::io::prelude::*;

use std::fs::{File};

#[cfg(test)]
mod tests{
    #[test]
    fn it_works(){
        assert_eq!(2 + 2, 4);
    }
}

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

        let (tx, rx) = channel();
        let iter = FileIter{ q:q };

        for s in iter.filter(|i| self.file_ok(i)) {
            let tx = tx.clone();  // Shadow tx from outer scope
            let q = self.query.clone();


            self.pool.execute(move || {
                // Expensive file search computations here
                let path = s;  

                let mut f = File::open(&path).expect(&format!("Could not open file: {}", path));
                let mut content = String::new();
                let _res = f.read_to_string(&mut content);

                content.lines().enumerate().filter(|(_i, line)| q.is_match(line))
                    .for_each(|(i, line)| {
                        let to_send = format!("\n{}: line {} \n{}", path, i, line);
                        tx.send(to_send).expect("Weird code crash");
                    });
            });


        }
        Self::drop_tx(tx);

        self.pool.join();
        // Print all received strings
        for s in rx{
            println!("{}", s);
        }
    }

    fn file_ok(&self, f: &str) -> bool{
        self.files.is_match(f)
    }

    fn drop_tx(_tx: Sender<String>) {}
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

