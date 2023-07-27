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
use std::{cell::RefCell, fs, rc::Rc, time};

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

type FilePtr = usize;

#[derive(Debug)]
struct File {
    name: String,
    parent_file: Option<FilePtr>,
    children_files: Option<Vec<FilePtr>>,
    size: i32,
    location: FilePtr,
}

impl File {
    fn new(name: &str, parent_file: Option<FilePtr>, location: FilePtr, size: i32) -> Box<File> {
        Box::new(File {
            name: String::from(name),
            parent_file,
            children_files: None,
            size,
            location,
        })
    }

    fn is_dir(&self) -> bool {
        !self.children_files.is_none()
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn update_size(&mut self, size: i32) {
        self.size += size;
    }

    fn get_size(&self) -> i32 {
        self.size.clone()
    }

    fn get_location(&self) -> FilePtr {
        self.location.clone()
    }

    fn get_parent(&self) -> Option<FilePtr> {
        self.parent_file.clone()
    }

    fn get_children(&self) -> Option<&Vec<FilePtr>> {
        if self.children_files.is_none() {
            return None;
        }
        self.children_files.as_ref()
    }

    fn push_child(&mut self, file_location: FilePtr) {
        if self.children_files.is_none() {
            self.children_files = Some(vec![])
        }
        self.children_files.as_mut().unwrap().push(file_location);
    }
}

#[derive(Debug)]
struct SSD {
    curr_dir: FilePtr,
    files: Vec<Box<File>>,
}

impl SSD {
    fn new() -> SSD {
        let root = File::new("/", None, 0, 0);
        let files = vec![root];

        SSD { curr_dir: 0, files }
    }

    fn update_size(&mut self, location: usize, size: i32) {
        let mut next_location = location;
        loop {
            self.files.get_mut(next_location).unwrap().update_size(size);
            if let Some(parent )= self.files.get(next_location).unwrap().get_parent() {
                next_location = parent;
                continue;
            }
            return;
        }
    }

    fn get_all_files(&self) -> &Vec<Box<File>> {
        &self.files
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
        self.print_disk(self.curr_dir, 1);
    }

    fn exec_mkdir(&mut self, dir_name: &String) {
        let new_location = self.files.len();
        let file = File::new(&dir_name, Some(self.curr_dir), new_location, 0);
        self.files.push(file);
        self.files
            .get_mut(self.curr_dir)
            .unwrap()
            .push_child(new_location);
    }

    fn exec_touch(&mut self, file_name: &str, size: i32) {
        let new_location = self.files.len();
        let file = File::new(&file_name, Some(self.curr_dir), new_location, size);
        self.files.push(file);
        self.update_size(self.curr_dir, size);
        self.files
            .get_mut(self.curr_dir)
            .unwrap()
            .push_child(new_location);
    }

    fn set_curr_dir_to_root(&mut self) {
        self.curr_dir = 0;
    }

    fn move_to_prev_dir(&mut self) {
        let curr_file = self.files.get(self.curr_dir).unwrap();
        let parent_location = curr_file.get_parent();
        if parent_location.is_none() {
            return;
        }
        self.curr_dir = parent_location.unwrap();
    }

    fn move_to_child_dir(&mut self, dir_name: &String) {
        let curr_file = self.files.get(self.curr_dir).unwrap();
        let children = curr_file.get_children();
        if children.is_none() {
            return;
        }

        let child = children
            .unwrap()
            .iter()
            .map(|file_location| self.files.get(file_location.clone()))
            .filter(|file| !file.is_none())
            .map(|file| file.unwrap())
            .find(|file| file.get_name() == dir_name);
        if child.is_none() {
            return;
        }

        self.curr_dir = child.unwrap().get_location();
    }

    fn print_disk(&self, file_ptr: FilePtr, depth: usize) {
        let file = self.files.get(file_ptr).unwrap();
        let name = file.get_name().clone();
        let size = file.get_size();
        println!("- {name} {size}");

        let children = file.get_children();
        if children.is_none() {
            return;
        }

        for child in children.unwrap().iter() {
            print!("{}", "  ".repeat(depth));
            self.print_disk(child.clone(), depth + 1)
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
    let start_time = time::Instant::now();

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

    let ans = disk
        .get_all_files()
        .iter()
        .filter(|file| file.is_dir())
        .filter(|file| file.get_size() <= 100000)
        .map(|file| file.get_size())
        .sum::<i32>();

    let elapsed = start_time.elapsed().as_micros();
    println!("Elapsed: {}", elapsed);

    ans
}
