/////////////////////////////////////////////////////////////
// rust_textfinder::main.rs

#![allow(unused_variables)]
#![allow(dead_code)]
use std::io::prelude::*;

#[derive(Debug, Default)]
pub struct TextFinder {
    re_str : String,
}
impl TextFinder {
    pub fn new() -> TextFinder {
        Self { re_str: String::default(), }
    }
    pub fn regex(&mut self, s:&str) {
        self.re_str = s.to_string();
    }
    pub fn find(&self, file_path: &str, regex:&str) -> bool {
        let rslt = std::fs::read_to_string(file_path);
        match rslt {
            Ok(contents) => {
               let rx_rslt = regex::Regex::new(regex);
               match rx_rslt {
                   Ok(re) => return re.is_match(&contents),
                   Err(_) => false,
               }
            },
            Err(_) => return false,
        }
    }
}
fn test_setup() -> std::io::Result<std::fs::File> {
    let mut file = std::fs::File::create("test1.txt")?;
    let _ = file.write_all(b"abcdefg");
    let mut file = std::fs::File::create("test2.txt")?;
    let _ = file.write_all(b"foobar");
    Ok(file)
}
fn announce(fp: &str, re:&str, r:bool) {
    if r {
        print!("\n  found regex {:?} match in file {:?}", re, fp);
    }
    else {
        print!("\n  didn't find regex {:?} match in file {:?}", re, fp);
    }
}
fn test_file(file_path: &str, regex:&str) {
    let tf = TextFinder::new();
    let found = tf.find(file_path, regex);
    announce(file_path, regex, found);
}
fn test() {
    let rslt = test_setup();
    match rslt {
        Ok(_) => {
            test_file("test1.txt", "bcd");
            test_file("test1.txt", "xyz");
            test_file("test2.txt", "foo");
            test_file("no_exist", ".");
        }
        Err(_) => {
            print!("\n  test_setup() failed, quitting\n\n");
            return;
        }
    }
}

#[derive(Debug, Default)]
pub struct TfAppl {
    re_str: String,
    tf: TextFinder,
}
impl rust_dir_nav::DirEvent for TfAppl {
    fn do_dir(&mut self, d:&str) {
        print!("\n  {:?}", d);
    }
    fn do_file(&mut self, f:&str) {
        self.tf.regex(&self.re_str);
        if self.tf.find(f, &self.re_str) {
            print!("\n  found regex {} in {}", self.re_str, f);
        }
        print!("\n    {:?}", f);
    }
}
impl TfAppl {
    pub fn new() -> Self {
        Self {
            re_str: String::default(),
            tf: TextFinder::new(),
        }
    }
    pub fn regex(&mut self, s:&str) {
        self.re_str = s.to_string();
    }
}

fn main() {
    let do_test = false;
    if do_test {
        test();
    }
    else {
        let mut parser = rust_cmd_line::CmdLineParse::new();
        parser.parse();
        parser.add_pattern("rs").add_pattern("txt");
    
        //type Appl = rust_dir_nav::Appl;
        let mut dn = rust_dir_nav::DirNav::<TfAppl>::new();
        dn.get_app().regex(".");

        for patt in parser.patterns() {
            dn.add_pat(patt);
        }
        let mut p = std::path::PathBuf::new();
        p.push(parser.abs_path());
        let _rslt = dn.visit(&p);
    }
    println!("\n\n  That's all Folks!\n\n");
}
