use std::io;
use std::io::prelude::*;

struct KMPMatch {
    t: Vec<i32>,
    pat: String
}

impl KMPMatch {
    pub fn new(pattern: String) -> KMPMatch {
        let mut vec: Vec<i32> = vec![0; pattern.len() + 1];
        let mut i = 0;
        let mut j: i32 = -1;
        vec[i] = j;
        while i < pattern.len() {
            while j >= 0 && pattern.as_bytes()[i] != pattern.as_bytes()[j as usize] {
                j = vec[j as usize];
            }
            i += 1;
            j += 1;
            vec[i] = j;
        }

        KMPMatch {
            t: vec,
            pat: pattern
        }
    }

    pub fn find(&self) -> Vec<i32> {
        let txt: String = String::from(self.pat[1..].to_string());
        let mut m = 0;
        let mut i = 0;
        let mut matches: Vec<i32> = vec![];

        loop {
            if m + i >= txt.len() || self.pat.as_bytes()[i] == txt.as_bytes()[m + i] {
                if i == (self.pat.len() - 1) {
                    matches.push(m as i32);
                    return matches;
                }
                i += 1;
            } else {
                if self.t[i] != -1 {
                    m = m + i - self.t[i] as usize;
                    i = self.t[i] as usize;
                } else {
                    i = 0;
                    m += 1;
                }
            }
        }
    }
}

fn main() {
    let input = io::stdin();

    let mut lines =  input.lock().lines().map(|_line| _line.ok().unwrap());
    let _n = lines.next();

    let a: String = lines.next().unwrap();
    let kmp = KMPMatch::new(a);
    let matches: Vec<i32> = kmp.find();

    println!("{}", matches[0] + 1);
}
