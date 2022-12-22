use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::reader::read_aoc_file;

pub fn day7() {
    if let Ok(lines) = read_aoc_file(7) {

        let root = Rc::new(RefCell::new(
            Dir { name: String::from("/"), files: vec![], dirs: vec![], parent: None }
        ));
        let mut current_dir: Rc<RefCell<Dir>> = Rc::clone(&root);

        for line in lines {
            if let Ok(data) = line {
                if data.starts_with("$") {
                    current_dir = run_command(data, Rc::clone(&root), Rc::clone(&root));
                } else if data.starts_with("dir") {
                    store_dir(data, Rc::clone(&root));
                } else {
                    //current_dir.files.push(store_file(data));
                }
            }
        }

        println!("{:?}", root);
    }
}


fn run_command(data: String, root: Rc<RefCell<Dir>>, current_dir: Rc<RefCell<Dir>>) -> Rc<RefCell<Dir>> {
    if data.starts_with("$ ls") {
        // no-op
    } else if data.starts_with("cd") {
        let cd_dir = data.replace("$ cd ", "");
        if cd_dir == "/" {
            return root;
        }
        let cur_dir = current_dir.borrow_mut();

        let cd_to = cur_dir.dirs.iter().find(|d| d.name == cd_dir);
        match cd_to {
            None => { panic!("No known dir {} within {}", cd_dir, cur_dir.name)}
            Some(dir) => {
                return Rc::clone(dir);
            }
        }
    } else {
        panic!("Unknown command {}", data)
    }
    return current_dir;
}

fn store_dir(data: String, current_dir: Rc<RefCell<Dir>>) {
    let dir_name = data.replace("dir ", "");
    if !current_dir.borrow().borrow().has_child(dir_name) {
        //let dir = Dir { name: , files: vec![], dirs: vec![], parent: Some(current_dir) };
        let new_dir = Rc::new(RefCell::new(
            Dir { name: dir_name.clone(), files: vec![], dirs: vec![], parent: Some(current_dir) }
        ));
        current_dir.borrow().borrow().dirs.push(dir);
        println!("Stored {} under {}", dir_name, current_dir.borrow().borrow().name);
    }
}

fn store_file(data: String) -> File {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn has_child(&self, name: String) -> bool {
        self.dirs.iter().any(|d| d.name == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: i32,
}