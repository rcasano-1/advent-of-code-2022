use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone, Debug, Default)]
struct FsEntry {
    path: String,
    size: u64, // value of 0 if it is a directory
    children: HashMap<String, Rc<RefCell<FsEntry>>>,
}

impl FsEntry {
    pub fn is_dir(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u64 {
        let sum = Cell::new(0u64);
        self.visit(|fs: &FsEntry| {
            sum.set(sum.get() + fs.size);
        });
        sum.get()
    }

    pub fn add_file(&mut self, path: &str, size: u64) {
        self.children
            .entry(String::from(path))
            .or_insert(Rc::new(RefCell::new(Self {
                path: String::from(path),
                size,
                ..Self::default()
            })));
    }

    pub fn add_or_get_dir(&mut self, path: &str) -> Rc<RefCell<FsEntry>> {
        let new_dir = Rc::new(RefCell::new(Self {
            path: String::from(path),
            ..Self::default()
        }));
        Rc::clone(self.children.entry(String::from(path)).or_insert(new_dir))
    }

    pub fn visit<F>(&self, mut func: F)
    where
        F: FnMut(&FsEntry) + Copy,
    {
        self.children
            .values()
            .for_each(|x| x.borrow_mut().visit::<F>(func));
        func(self);
    }

    fn print_tree_helper(&self, depth: usize) {
        for _ in 0..depth {
            print!(" ");
        }
        print!("- {}", self.path);
        if self.is_dir() {
            print!("/ [{}]", self.size());
            println!();
            self.children
                .values()
                .for_each(|x| Rc::clone(x).borrow().print_tree_helper(depth + 4));
        } else {
            print!(" [{}]", self.size());
            println!();
        }
    }

    #[allow(dead_code)]
    pub fn print_tree(&self) {
        self.print_tree_helper(0)
    }
}

#[allow(dead_code)]
fn path_to_string(cur_path: &Vec<Rc<RefCell<FsEntry>>>) -> String {
    cur_path
        .iter()
        .map(|x| Rc::clone(x).borrow_mut().path.clone())
        .collect::<Vec<_>>()
        .join("/")
}

fn main() {
    let input = include_str!("my_input.txt");

    // Top of FileSystem
    let mut cur_working_dir = Rc::new(RefCell::new(FsEntry {
        path: "<fs>".to_string(),
        ..FsEntry::default()
    }));
    let filesystem = Rc::clone(&cur_working_dir);

    // Current path stack
    let mut cur_path: Vec<Rc<RefCell<FsEntry>>> = Vec::new();
    cur_path.push(Rc::clone(&cur_working_dir));

    // Former implementation:
    /*
        let root = Rc::new(RefCell::new(FsEntry {
            path: "/".into(),
            ..FsEntry::default()
        }));
        let mut working_dir_stack = vec![&root];
        let mut cur_dir = Rc::clone(&root);
    */

    // Collect files
    for line in input.lines() {
        if line.starts_with("$") {
            let cmd = &line[2..];

            if cmd.starts_with("ls") {
                // ignore this case
            } else if cmd.starts_with("cd") {
                if let Some((_cd, dir_name)) = cmd.split_once(' ') {
                    match dir_name {
                        "/" => {
                            continue;
                        }
                        ".." => {
                            // return to parent
                            cur_path.pop();
                            let new_working_dir = cur_path.last().unwrap();
                            cur_working_dir = Rc::clone(new_working_dir);
                        }
                        _ => {
                            // change into subdirectory
                            let new_working_dir =
                                cur_working_dir.borrow_mut().add_or_get_dir(dir_name);
                            cur_path.push(Rc::clone(&new_working_dir));
                            cur_working_dir = new_working_dir;
                        }
                    }
                }

                // Former implementation:
                /*
                if line.contains("/") {
                    // ignore this case, root directory
                } else if line.contains("..") {
                    // handle stack pop operation
                    //println!("Child: {:#?}", child);

                    // stack.last_mut().unwrap().children.push(child.unwrap());
                } else {
                    // handle change directory: $ cd <dir>
                    let path = line
                        .split_whitespace()
                        .into_iter()
                        .last()
                        .unwrap()
                        .to_string();

                    if working_dir_stack
                        .last()
                        .unwrap()
                        .borrow()
                        .children
                        .contains_key(&path)
                    {
                    } else {
                        let node = Rc::new(RefCell::new(FsEntry {
                            path: path.clone(),
                            ..FsEntry::default()
                        }));

                        let new_dir_parent = working_dir_stack.last_mut().unwrap();
                        new_dir_parent
                            .borrow_mut()
                            .children
                            .insert(path.clone(), Rc::clone(&node));

                        working_dir_stack.push(Rc::clone(&&node));

                        println!("Node ref count: {}", Rc::strong_count(&node));

                        dbg!(&working_dir_stack);
                    }

                    // .insert(child.as_ref().unwrap().path.clone(), child.unwrap());
                    //println!("{:#?}", node);
                }
                */
            } else {
                panic!("Unknown command: {}", cmd);
            }
        } else {
            // Command output
            // Process a file name and its size: 14848514 b.txt
            // Handle directories
            let (start, rest) = line.split_once(" ").unwrap();
            match start {
                "dir" => {
                    let filename = rest.to_string();
                    cur_working_dir.borrow_mut().add_or_get_dir(&filename);
                }
                _ => {
                    let size = start.parse().unwrap();
                    let filename = rest;
                    cur_working_dir.borrow_mut().add_file(filename, size);
                }
            }

            // Former implementation:
            /*
            let size = line.split_once(" ").unwrap().0.parse().unwrap();
            let path = line.split_once(" ").unwrap().1.to_string();
            let node = Rc::new(RefCell::new(FsEntry {
                size,
                path: path.clone(),
                ..FsEntry::default()
            }));
            // println!("{:#?}", node);
            Rc::clone(&cur_dir)
                .borrow_mut()
                .children
                .insert(path.clone(), node);
            */
        }
    }
    filesystem.borrow().print_tree();

    // Part 1 - Find directories with total size less than 100K
    let dir_sizes = RefCell::new(Vec::new());
    filesystem.borrow().visit(|fs| {
        if fs.is_dir() {
            dir_sizes
                .borrow_mut()
                .push((fs.path.to_string(), fs.size()));
        }
    });
    let sum = dir_sizes
        .borrow()
        .iter()
        .filter(|(_name, size)| *size < 100000)
        .fold(0, |a, e| a + e.1);
    println!();
    println!("Part 1 - Sum of total sizes: {sum}");

    // Part 2 - Choose a directory to delete
    let total_disk_space = 70000000;
    let need_to_free = 30000000;
    let fs_size = filesystem.borrow().size();
    let cur_free = total_disk_space - fs_size;
    let mut dir_sizes_part2 = dir_sizes
        .borrow()
        .iter()
        .filter(|x| cur_free + x.1 > need_to_free)
        .map(|x| x.1)
        .collect::<Vec<_>>();
    dir_sizes_part2.sort();
    println!(
        "Part 2 - Smallest (directory size) to be deleted: {}",
        dir_sizes_part2[0]
    );
}
