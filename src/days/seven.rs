use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day7/input");
    let fs = FS {
        root: Rc::new(RefCell::new(Directory::new("/".to_string()))),
    };
    let mut current_dir = fs.root.clone();
    for line in lines.iter().skip(1) {
        let mut split = line.splitn(3, " ");
        let (x, y, z) = (split.next().unwrap(), split.next().unwrap(), split.next());
        match (x, y, z) {
            ("$", "ls", _) => (),
            ("$", "cd", Some("..")) => {
                let parent = current_dir.borrow().parent.as_ref().unwrap().clone();
                current_dir = parent.upgrade().unwrap();
            }
            ("$", "cd", Some(target)) => {
                let child = current_dir
                    .borrow()
                    .child_dirs
                    .iter()
                    .find(|d| d.borrow().name == target)
                    .unwrap()
                    .clone();
                current_dir = child;
            }
            ("dir", target, _) => current_dir
                .borrow_mut()
                .add_child(target.to_string(), &current_dir),
            (size, name, _) => current_dir.borrow_mut().add_file(File::new(size, name)),
        }
    }
    let mut du = vec![];
    let total_size = fs.disk_usage(fs.root.clone(), &mut du);
    let res1 = du.iter().filter(|size| **size <= 100000).sum::<usize>();
    let available_space = 70000000 - total_size;
    let res2 = du
        .into_iter()
        .filter(|&s| available_space + s >= 30000000)
        .min()
        .unwrap();

    println!("RESULTS\nP1: {}\nP2: {}", res1, res2);
    Ok(())
}

struct FS<'a> {
    root: Rc<RefCell<Directory<'a>>>,
}
impl<'a> FS<'a> {
    fn disk_usage(&self, node: Rc<RefCell<Directory<'a>>>, result: &mut Vec<usize>) -> usize {
        let dir_total = node
            .borrow()
            .child_dirs
            .iter()
            .fold(0, |acc, d| acc + self.disk_usage(d.clone(), result));
        let file_sum = node.borrow().files.iter().fold(0, |acc, f| acc + f.size);
        result.push(file_sum + dir_total);
        file_sum + dir_total
    }
}
struct Directory<'a> {
    name: String,
    parent: Option<Weak<RefCell<Directory<'a>>>>,
    child_dirs: Vec<Rc<RefCell<Directory<'a>>>>,
    files: Vec<File>,
}

impl<'a> Directory<'a> {
    fn new(name: String) -> Self {
        Self {
            name,
            parent: None,
            child_dirs: vec![],
            files: vec![],
        }
    }
    fn add_child(&mut self, name: String, self_wrapped: &Rc<RefCell<Self>>) {
        self.child_dirs.push(Rc::new(RefCell::new(Directory {
            name,
            parent: Some(Rc::downgrade(self_wrapped)),
            child_dirs: vec![],
            files: vec![],
        })))
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}
struct File {
    size: usize,
    _name: String,
}
impl File {
    fn new(size: &str, name: &str) -> Self {
        Self {
            size: size.parse().unwrap(),
            _name: name.to_string(),
        }
    }
}
