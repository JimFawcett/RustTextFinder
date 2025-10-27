# Rust TextFinder


[RustTextFinder code](https://github.com/JimFawcett/RustTextFinder)

# RustTextFinder  Repository

### Search for files containing specified text

Quick Status Version 1.1.0 Code status No known defects Demonstration code NA Documentation yes Test cases NA Static library NA Build requires Rust installation Planned design changes None for now

### Concept:

RustTextFinder is a tool for locating files containing text that matches a specified regular expression. It uses the facilities of RustDirNav and RustCmdLine libraries, and std::fs and regex crates. It can be used to:

1.  Find all files in a directory subtree with text that matches a specified regular expression.
2.  Find all files that have specified extensions (patterns).
3.  List all directories and their files in some directory subtree.

It processes many hundreds of files and directories in a few seconds.

<img src="https://github.com/JimFawcett/JimFawcett.github.io/blob/master/Pictures/RustTextFinder.jpg" style="float:right; margin-left:10px; width:45rem;">
<!-- ![class diagram](https://github.com/JimFawcett/JimFawcett.github.io/blob/master/Pictures/RustTextFinder.jpg) -->

### Design:

TextFinder implements the following methods and functions:

1.  **new() -> TextFinder**
    
    Create new TextFinder which holds a regex string.
    
2.  **find(&self, file\_path: &str) -> bool**
    
    returns true if file content matches internal regex string.
    
3.  **regex(&mut self, re:&str)**
    
    Replace current regex string with re.
    
4.  **get\_regex(&self) -> &str**
    
    Retrieve regex string.
    

RustTextFinder uses the services TextFinder, described above, and RustDirNav::DirNav<App>, where App must implement the trait RustDirNav::DirEvent. For this application, DirNav is declared with the user-defined type TfAppl for its App parameter. TfAppl holds an instance of TextFinder, a curr\_dir string, and predicates hide and recurse. When DirNav<TfAppl> encounters a file or directory it calls the DirEvent functions do\_dir(&mut self, d:&str) and do\_file(&mut self, f:&str). TfAppl defines those functions to build output displays based on its command line parameters, as shown in Fig 2.

### Operation:

When RustTextFinder is started, it:

1.  creates an instance of CmdLineParser parser
2.  uses that to evaluate the program's option parameters.
3.  It then creates an instance of DirNav<TfAppl> dn, supplying it with the starting path, file patterns, and recursion predicate, all extracted from parser.
4.  calls dn.visit, starting at parser.abs\_path(), converted to a std::Path.

The results are shown in Fig 2.

### Build:

Download and, in a command prompt, cargo build or cargo run.

### Status:

Version 1.1.0  
Tested on both Windows 10 and Ubuntu Linux

Rust Repositories

[Logger](RustLogger.html)  [TextFinder](RustTextFinder.html)  [BlockingQueue](RustBlockingQueue.html)  [ThreadPool](RustThreadPool.html)  [CmdLineParser](RustCmdLine.html)  [DisplayLib](RustDisplayLib.html)  [DirNav](RustDirNav.html)  [Comm](RustComm.html)   [Comm w/TP](RustCommWithThreadPool.html)  [String Conver](RustStringConversions.html)  [BuildOn](BuildOn.html)   [Comm Exper's](RustCommExperiments.html)  [Byte Record](RustByteRecord.html)  [Basic Demos](RustBasicDemos.html)  [Error Handling](RustErrorHandling.html)  [Rust Models](RustModels.html)   [Library Demos](RustLibraryDemos.html)  [Rust Bite by Byte](RustBiteByByte.html)        [bottom](#bottom) [oper](#oper) [design](#design) [concept](#concept) [top](#top)

