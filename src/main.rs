/////////////////////////////////////////////////////////////
// rust_textfinder::main.rs - recursive search for text    //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 05 May 2020  //
/////////////////////////////////////////////////////////////
/*
   TextFinder 
   - Recursive search for regular expression matches in
     files in a directory at a specified root path.
   - Options
       /P "..."       path to root of search
       /p "rs, txt"   file patterns to search
       /r "abc|BCD"   regular expression to match
       /s [true]      recursive search
       /H [true]      hide paths with no matches
       /v [true]      show options
*/
#![allow(unused_variables)]
#![allow(dead_code)]

/*-- TextFinder searches for text that matches a regex --*/
#[derive(Debug, Default)]
pub struct TextFinder {
    re_str : String,
    last_dir : String,
    hide : bool,
    recurse : bool,
}
impl TextFinder {
    pub fn new() -> TextFinder {
        Self { 
            re_str: String::default(), 
            last_dir: String::default(),
            hide: false,
            recurse: false,
        }
    }
    pub fn regex(&mut self, s:&str) {
        self.re_str = s.to_string();
    }
    pub fn get_regex(&self) -> &str {
        &self.re_str
    }
    pub fn last_path(&mut self, p:&str) {
        self.last_dir = p.to_string();
    }
    pub fn get_last_path(&self) -> &str {
        &self.last_dir
    }
    pub fn hide(&mut self, p:bool) {
        self.hide = p;
    }
    pub fn get_hide(&self) -> bool {
        self.hide
    }
    pub fn recurse(&mut self, p:bool) {
        self.recurse = p;
    }
    pub fn get_recurse(&self) -> bool {
        self.recurse
    }
    pub fn find(&self, file_path: &str) -> bool {
        let contents:String;
        /*-- attempt to read file as text --*/
        let txt_rslt = std::fs::read_to_string(file_path);
        match txt_rslt {
            Ok(text) => contents = text,
            Err(_) => {
                /*-- attempt to read file as bytes --*/
                let bytes_rslt = std::fs::read(file_path);
                match bytes_rslt {
                    Ok(bytes) => {
                        /*-- convert bytes to lossy string --*/
                        contents = String::from_utf8_lossy(&bytes).to_string();
                    },
                    Err(_) => { return false; },
                }
            }
        }
        let rx_rslt = regex::Regex::new(&self.re_str);
        match rx_rslt {
            Ok(re) => return re.is_match(&contents),
            Err(_) => false,
        }
    }
}

/*-- TfAppl is an application specific proxy for TextFinder --*/
#[derive(Debug, Default)]
pub struct TfAppl {
    tf: TextFinder,
    curr_dir: String,
}
impl rust_dir_nav::DirEvent for TfAppl {
    fn do_dir(&mut self, d:&str) {
        /*-- save dir name for use in do_file --*/
        self.curr_dir = d.to_string();
        /*-- print directory name if H(ide) is false --*/
        if !self.tf.get_hide() {
            print!("\n--{}", d);
        }
    }
    fn do_file(&mut self, f:&str) {
        /*-- build fully qualified path to file --*/
        let mut fqf = self.curr_dir.clone();
        fqf.push('/');
        fqf.push_str(f);
        /*-- look for file text that matches regex --*/
        if self.tf.find(&fqf) {
            /*-- print directory for first file if H(ide) is true --*/
            let pred = 
              self.tf.get_last_path() != self.curr_dir 
              && self.tf.get_hide()==true;
            if  pred {
                print!("\n\n  {}", self.curr_dir);
                self.tf.last_path(&self.curr_dir);
            }
            /*-- print name of file with matching text --*/
            print!("\n      {:?}", f);
        }
    }
}
impl TfAppl {
    pub fn new() -> Self {
        Self {
            tf: TextFinder::new(),
            curr_dir: String::default(),
        }
    }
    pub fn recurse(&mut self, p:bool) {
        self.tf.recurse(p);
    }
    pub fn get_recurse(&self) -> bool {
        self.tf.get_recurse()
    }
    pub fn hide(&mut self, p:bool) {
        self.tf.hide(p);
    }
    pub fn get_hide(&self) -> bool {
        self.tf.get_hide()
    }
    pub fn regex(&mut self, s:&str) {
        self.tf.regex(s);
    }
    pub fn get_regex(&self) -> &str {
        self.tf.get_regex()
    }
}
/*-- display title, display options if v(erbose) is true --*/
fn verbose(parser: &rust_cmd_line::CmdLineParse) {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    print!("\n  TextFinder ver {}",VERSION);
    print!("\n =======================");
    if parser.options().contains_key(&'v') {
        print!("\n  path = {}", parser.abs_path());
        print!("\n  patterns = ");
        for patt in parser.patterns() {
            print!("{:?} ", patt);
        }
        print!("\n  regex = {:?}", parser.get_regex());
        for key in parser.options().keys() {
            let value_option = parser.options().get(&key);
            match value_option {
                Some(value) => {
                    print!("\n  option: {} {:?}", key, value);
                },
                None => (),
            }
        }
    }
    else {
        print!("\n  searching path: {:?}", &parser.abs_path());
        print!("\n  matching files with regex: {:?}", parser.get_regex());
    }
}

fn main() {
    // let do_test = false;
    // if do_test {
    //     test2();
    // }
    // else {
        let mut parser = rust_cmd_line::CmdLineParse::new();
        parser.default_options();
        parser.parse();
    
        let mut dn = rust_dir_nav::DirNav::<TfAppl>::new();
        // dn.hide(false);
        //let app = dn.get_app();

        if parser.options().contains_key(&'s') {
            let r_value = parser.options()[&'s'] == "true";
            dn.recurse(r_value);
            dn.get_app().recurse(r_value);
        }
        else {
            dn.recurse(false);
            dn.get_app().recurse(false);
        }

        if parser.options().contains_key(&'H') {
            let h_value = parser.options()[&'H']=="true";
            dn.hide(h_value);
            dn.get_app().hide(h_value);
        }
        else {
            dn.hide(true);
            dn.get_app().hide(true);
        }

        dn.get_app().regex(parser.get_regex());

        for patt in parser.patterns() {
            dn.add_pat(patt);
        }
        let mut p = std::path::PathBuf::new();
        p.push(parser.abs_path());

        verbose(&parser);
        let _ = dn.visit(&p);

        print!(
            "\n\n  processed {} files in {} dirs", 
            dn.get_files(), dn.get_dirs()
        );
    // }
    println!("\n\n  That's all Folks!\n\n");
}
