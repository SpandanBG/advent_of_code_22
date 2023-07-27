/**
 * --- Day 7: No Space Left On Device ---
 * You can hear birds chirping and raindrops hitting leaves as the expedition
 * proceeds. Occasionally, you can even hear much louder sounds in the distance;
 * how big do the animals get out here, anyway?
 *
 * The device the Elves gave you has problems with more than just its
 * communication system. You try to run a system update:
 *
 * $ system-update --please --pretty-please-with-sugar-on-top
 * Error: No space left on device
 * Perhaps you can delete some files to make space for the update?
 *
 * You browse around the filesystem to assess the situation and save the
 * resulting terminal output (your puzzle input). For example:
 *
 * $ cd /
 * $ ls
 * dir a
 * 14848514 b.txt
 * 8504156 c.dat
 * dir d
 * $ cd a
 * $ ls
 * dir e
 * 29116 f
 * 2557 g
 * 62596 h.lst
 * $ cd e
 * $ ls
 * 584 i
 * $ cd ..
 * $ cd ..
 * $ cd d
 * $ ls
 * 4060174 j
 * 8033020 d.log
 * 5626152 d.ext
 * 7214296 k
 *
 * The filesystem consists of a tree of files (plain data) and directories
 * (which can contain other directories or files). The outermost directory is
 * called /. You can navigate around the filesystem, moving into or out of
 * directories and listing the contents of the directory you're currently in.
 *
 * Within the terminal output, lines that begin with $ are commands you executed,
 * very much like some modern computers:
 *
 * cd means change directory. This changes which directory is the current
 * directory, but the specific result depends on the argument:
 * cd x moves in one level: it looks in the current directory for the directory
 * named x and makes it the current directory.
 * cd .. moves out one level: it finds the directory that contains the current
 * directory, then makes that directory the current directory.
 * cd / switches the current directory to the outermost directory, /.
 * ls means list. It prints out all of the files and directories immediately
 * contained by the current directory:
 * 123 abc means that the current directory contains a file named abc with
 * size 123.
 * dir xyz means that the current directory contains a directory named xyz.
 * Given the commands and output in the example above, you can determine that
 * the filesystem looks visually like this:
 *
 * - / (dir)
 *   - a (dir)
 *     - e (dir)
 *       - i (file, size=584)
 *     - f (file, size=29116)
 *     - g (file, size=2557)
 *     - h.lst (file, size=62596)
 *   - b.txt (file, size=14848514)
 *   - c.dat (file, size=8504156)
 *   - d (dir)
 *     - j (file, size=4060174)
 *     - d.log (file, size=8033020)
 *     - d.ext (file, size=5626152)
 *     - k (file, size=7214296)
 *
 * Here, there are four directories: / (the outermost directory), a and d
 * (which are in /), and e (which is in a). These directories also contain files
 * of various sizes.
 *
 * Since the disk is full, your first step should probably be to find
 * directories that are good candidates for deletion. To do this, you need to
 * determine the total size of each directory. The total size of a directory is
 * the sum of the sizes of the files it contains, directly or indirectly.
 * (Directories themselves do not count as having any intrinsic size.)
 *
 * The total sizes of the directories above can be found as follows:
 *
 * The total size of directory e is 584 because it contains a single file i of
 * size 584 and no other directories.
 * The directory a has total size 94853 because it contains files f (size 29116),
 * g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e
 * which contains i).
 *
 * Directory d has total size 24933642.
 *
 * As the outermost directory, / contains every file. Its total size is 48381165,
 * the sum of the size of every file.
 * To begin, find all of the directories with a total size of at most 100000,
 * then calculate the sum of their total sizes. In the example above, these
 * directories are a and e; the sum of their total sizes is 95437 (94853 + 584).
 * (As in this example, this process can count files more than once!)
 *
 * Find all of the directories with a total size of at most 100000. What is the
 * sum of the total sizes of those directories?
*/
use regex::Regex;
use std::{cell::RefCell, fs, rc::Rc};

const ROOT_DIR_NAME: &'static str = "/";
const PREV_DIR_NAME: &'static str = "..";

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    Mkdir(String),
    Touch(i32, String),
}

impl Command {
    fn new(cmd_str: &str) -> Option<Command> {
        if let Some(cmd) = Command::create_cd_cmd(cmd_str) {
            return Some(cmd);
        }

        if let Some(cmd) = Command::create_ls_cmd(cmd_str) {
            return Some(cmd);
        }

        if let Some(cmd) = Command::create_mkdir_cmd(cmd_str) {
            return Some(cmd);
        }

        if let Some(cmd) = Command::create_touch_cmd(cmd_str) {
            return Some(cmd);
        }

        None
    }

    fn create_cd_cmd(cmd_str: &str) -> Option<Command> {
        let cd_cmd_regex = Regex::new(r"^\$ cd (.+)$").unwrap();
        if let Some(captures) = cd_cmd_regex.captures(cmd_str) {
            return Some(Command::Cd(String::from(&captures[1])));
        }
        None
    }

    fn create_ls_cmd(cmd_str: &str) -> Option<Command> {
        let ls_cmd_regex = Regex::new(r"^\$ ls$").unwrap();
        if ls_cmd_regex.is_match(cmd_str) {
            return Some(Command::Ls);
        }
        None
    }

    fn create_mkdir_cmd(cmd_str: &str) -> Option<Command> {
        let mkdir_cmd_regex = Regex::new(r"^dir (.+)$").unwrap();
        if let Some(captures) = mkdir_cmd_regex.captures(cmd_str) {
            return Some(Command::Mkdir(String::from(&captures[1])));
        }
        None
    }

    fn create_touch_cmd(cmd_str: &str) -> Option<Command> {
        let mkdir_cmd_regex = Regex::new(r"^([0-9]+) (.+)$").unwrap();
        if let Some(captures) = mkdir_cmd_regex.captures(cmd_str) {
            let size = String::from(&captures[1]).parse::<i32>().unwrap();
            let file_name = String::from(&captures[2]);
            return Some(Command::Touch(size, file_name));
        }
        None
    }
}

type FilePtr = Rc<RefCell<Box<File>>>;

#[derive(Debug)]
struct File {
    name: String,
    parent_file: Option<FilePtr>,
    children_files: Option<Vec<FilePtr>>,
    size: i32,
}

impl File {
    fn new(name: &str, parent_file: Option<FilePtr>, size: i32) -> Box<File> {
        Box::new(File {
            name: String::from(name),
            parent_file,
            children_files: None,
            size,
        })
    }

    fn is_dir(&self) -> bool {
        !self.children_files.is_none()
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_size(&self) -> i32 {
        self.size
    }

    fn get_parent(&self) -> Option<FilePtr> {
        if self.parent_file.is_none() {
            return None;
        }
        Some(Rc::clone(self.parent_file.as_ref().unwrap()))
    }

    fn get_children(&self) -> Option<&Vec<FilePtr>> {
        if self.children_files.is_none() {
            return None;
        }
        self.children_files.as_ref()
    }

    fn get_child(&self, file_name: &String) -> Option<FilePtr> {
        if self.children_files.is_none() {
            return None;
        }
        for file in self.children_files.as_ref().unwrap().iter() {
            if file.borrow().get_name().eq(file_name) {
                return Some(Rc::clone(file));
            }
        }
        return None;
    }

    fn push_child(&mut self, file: Box<File>) {
        if self.children_files.is_none() {
            self.children_files = Some(vec![])
        }

        self.update_size(file.size.clone());
        let file_ptr = Rc::new(RefCell::new(file));
        self.children_files.as_mut().unwrap().push(file_ptr);
    }

    fn update_size(&mut self, size: i32) {
        self.size += size;

        if !self.parent_file.is_none() {
            self.parent_file
                .as_mut()
                .unwrap()
                .borrow_mut()
                .update_size(size);
        }
    }
}

#[derive(Debug)]
struct SSD {
    root: FilePtr,
    curr_dir: FilePtr,
}

impl SSD {
    fn new() -> SSD {
        let root = File::new("/", None, 0);
        let root_ptr = Rc::new(RefCell::new(root));
        let curr_dir = Rc::clone(&root_ptr);

        SSD {
            root: root_ptr,
            curr_dir,
        }
    }

    fn exec_cd(&mut self, dir_name: &String) {
        if dir_name == ROOT_DIR_NAME {
            self.set_curr_dir_to_root();
            return;
        }

        if dir_name == PREV_DIR_NAME {
            self.move_to_prev_dir();
            return;
        }

        self.move_to_child_dir(&dir_name);
        return;
    }

    fn exec_ls(&self) {
        SSD::print_disk(Rc::clone(&self.curr_dir), 1);
    }

    fn exec_mkdir(&mut self, dir_name: &String) {
        let parent = Some(Rc::clone(&self.curr_dir));
        let new_dir = File::new(dir_name, parent, 0);
        self.curr_dir.borrow_mut().push_child(new_dir);
    }

    fn exec_touch(&mut self, file_name: &str, size: i32) {
        let parent = Some(Rc::clone(&self.curr_dir));
        let new_file = File::new(file_name, parent, size);
        self.curr_dir.borrow_mut().push_child(new_file);
    }

    fn set_curr_dir_to_root(&mut self) {
        self.curr_dir = Rc::clone(&self.root);
    }

    fn move_to_prev_dir(&mut self) {
        let parent = self.curr_dir.borrow().get_parent();
        if parent.is_none() {
            return;
        }

        self.curr_dir = Rc::clone(parent.as_ref().unwrap())
    }

    fn move_to_child_dir(&mut self, dir_name: &String) {
        let child = self.curr_dir.borrow().get_child(dir_name);
        if child.is_none() {
            return;
        }
        let child = child.unwrap();
        self.curr_dir = Rc::clone(&child);
    }

    fn print_disk(file: FilePtr, depth: usize) {
        let name = file.borrow().get_name().clone();
        let size = file.borrow().get_size();
        println!("- {name} {size}");

        let file_ptr = file.borrow();
        let children = file_ptr.get_children();
        if children.is_none() {
            return;
        }

        for child in children.unwrap().iter() {
            print!("{}", "  ".repeat(depth));
            SSD::print_disk(Rc::clone(&child), depth + 1)
        }
    }
}

pub fn get_inputs() -> Vec<Command> {
    fs::read_to_string("res/_7_no_space.txt")
        .unwrap()
        .split("\r\n")
        .map(|cmd_str| Command::new(cmd_str))
        .filter(|cmd| !cmd.is_none())
        .map(|cmd| cmd.unwrap())
        .collect::<Vec<Command>>()
}

// Part 1 sol

pub fn get_cleanable_space(inp: &Vec<Command>) -> i32 {
    let mut disk = SSD::new();

    for cmd in inp.iter() {
        match cmd {
            Command::Cd(dir_name) => disk.exec_cd(dir_name),
            Command::Mkdir(dir_name) => disk.exec_mkdir(dir_name),
            Command::Touch(size, file_name) => disk.exec_touch(file_name, *size),
            _ => (),
        };
    }

    // disk.exec_cd(&String::from(ROOT_DIR_NAME));
    // disk.exec_ls();

    get_required_dirs(Rc::clone(&disk.root))
        .into_iter()
        .filter(|size| *size <= 100000)
        .sum::<i32>()
}

fn get_required_dirs(file: FilePtr) -> Vec<i32> {
    if !file.borrow().is_dir() {
        return vec![];
    }

    let mut size_list = vec![file.borrow().get_size().clone()];

    let file_ptr = file.borrow();
    let children = file_ptr.get_children();
    if children.is_none() {
        return vec![]; // is a file 
    }

    for child in children.unwrap().iter() {
        size_list.extend(get_required_dirs(Rc::clone(&child)));
    }

    size_list
}