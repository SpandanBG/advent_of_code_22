use regex::Regex;

const ROOT_DIR_NAME: &'static str = "/";
const PREV_DIR_NAME: &'static str = "..";
const MAX_SSD_SPACE: i32 = 70000000;

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
pub struct File {
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
pub struct SSD {
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
            if let Some(parent) = self.files.get(next_location).unwrap().get_parent() {
                next_location = parent;
                continue;
            }
            return;
        }
    }

    fn get_used_space(&self) -> i32 {
        self.files.get(0).unwrap().get_size()
    }

    fn get_free_space(&self) -> i32 {
        MAX_SSD_SPACE - self.get_used_space()
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

    fn exec_ls(&self) {}

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
}

pub fn get_inputs(inp_raw: &str) -> Vec<Command> {
    inp_raw
        .split("\n")
        .map(|cmd_str| Command::new(cmd_str))
        .filter(|cmd| !cmd.is_none())
        .map(|cmd| cmd.unwrap())
        .collect::<Vec<Command>>()
}

pub fn setup_ssd(inp: &Vec<Command>) -> SSD {
    let mut disk = SSD::new();
    for cmd in inp.iter() {
        match cmd {
            Command::Cd(dir_name) => disk.exec_cd(dir_name),
            Command::Ls => disk.exec_ls(),
            Command::Mkdir(dir_name) => disk.exec_mkdir(dir_name),
            Command::Touch(size, file_name) => disk.exec_touch(file_name, *size),
        };
    }
    disk
}

// Part 1 sol
pub fn get_cleanable_space(disk: &SSD) -> i32 {
    disk.get_all_files()
        .iter()
        .filter(|file| file.is_dir())
        .filter(|file| file.get_size() <= 100000)
        .map(|file| file.get_size())
        .sum::<i32>()
}

// Part 2 sol
pub fn get_cleanable_space_v2(disk: &SSD) -> i32 {
    let required_total_free_space = 30000000;
    let requried_free_space = required_total_free_space - disk.get_free_space();

    disk.get_all_files()
        .iter()
        .filter(|file| file.is_dir())
        .filter(|file| file.get_size() >= requried_free_space)
        .map(|file| file.get_size())
        .fold(i32::MAX, |acc, curr| if curr < acc { curr } else { acc })
}
