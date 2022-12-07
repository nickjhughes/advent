use std::{
    cell::RefCell,
    fmt, fs,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq, Eq)]
enum DirectoryInput {
    UpOne,
    Home,
    Directory(String),
}

#[derive(Debug, PartialEq, Eq)]
enum Listing {
    Dir(String),
    File { name: String, size: u64 },
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    ChangeDirectory(DirectoryInput),
    List(Vec<Listing>),
}

#[derive(Debug)]
enum FileTree {
    Directory {
        name: String,
        parent: Option<Weak<RefCell<FileTree>>>,
        children: Vec<Rc<RefCell<FileTree>>>,
        level: usize,
    },
    File {
        name: String,
        size: u64,
        level: usize,
    },
}

impl fmt::Display for FileTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileTree::Directory {
                name,
                children,
                level,
                ..
            } => {
                let indent = vec![' '; *level * 2].iter().collect::<String>();
                writeln!(f, "{}- {} (dir)", indent, name)?;
                children.iter().fold(Ok(()), |result, child_tree| {
                    result.and_then(|_| write!(f, "{}", child_tree.borrow()))
                })
            }
            FileTree::File { name, size, level } => {
                let indent = vec![' '; *level * 2].iter().collect::<String>();
                writeln!(f, "{}- {:} (file, size={:})", indent, name, size)
            }
        }
    }
}

impl FileTree {
    fn new_root() -> Self {
        FileTree::Directory {
            name: "/".to_string(),
            parent: None,
            children: Vec::new(),
            level: 0,
        }
    }

    fn children(&self) -> std::slice::Iter<Rc<RefCell<FileTree>>> {
        match self {
            Self::Directory { children, .. } => children.iter(),
            _ => panic!("A FileNode::File has no children"),
        }
    }

    fn add_child(&mut self, child: FileTree) {
        match self {
            Self::Directory { children, .. } => {
                children.push(Rc::new(RefCell::new(child)));
            }
            _ => panic!("Can't add children to a FileNode::File"),
        }
    }

    fn find_child(&self, child_name: &str) -> Option<Rc<RefCell<FileTree>>> {
        match self {
            Self::Directory { children, .. } => {
                for child in children {
                    let child_data = child.borrow_mut();
                    match *child_data {
                        FileTree::Directory { ref name, .. } => {
                            if name == child_name {
                                return Some(child.clone());
                            }
                        }
                        FileTree::File { ref name, .. } => {
                            if name == child_name {
                                return Some(child.clone());
                            }
                        }
                    }
                }
                None
            }
            _ => panic!("A FileNode::File has no children"),
        }
    }

    fn from_commands(commands: &[Command]) -> Self {
        let root_node = Rc::new(RefCell::new(FileTree::new_root()));
        let mut current_node = root_node.clone();
        for command in commands {
            match command {
                Command::ChangeDirectory(directory_input) => match directory_input {
                    DirectoryInput::Home => {
                        current_node = root_node.clone();
                    }
                    DirectoryInput::UpOne => {
                        let new_parent = match &*current_node.borrow() {
                            FileTree::Directory { parent, .. } => {
                                parent.as_ref().map(|parent| parent.upgrade().unwrap())
                            }
                            _ => panic!(),
                        };
                        if let Some(new_parent) = new_parent {
                            current_node = new_parent;
                        }
                    }
                    DirectoryInput::Directory(name) => {
                        assert!(matches!(*current_node.borrow(), FileTree::Directory { .. }));

                        let directory = current_node
                            .borrow()
                            .find_child(name)
                            .expect("Can't `cd` into a directory we haven't seen in a listing yet");
                        current_node = directory;
                    }
                },
                Command::List(listings) => {
                    assert!(matches!(*current_node.borrow(), FileTree::Directory { .. }));

                    let current_level = match &*current_node.borrow() {
                        FileTree::Directory { level, .. } => *level,
                        _ => panic!(),
                    };

                    for listing in listings {
                        let child = match listing {
                            Listing::Dir(name) => FileTree::Directory {
                                name: name.clone(),
                                parent: Some(Rc::downgrade(&current_node)),
                                children: Vec::new(),
                                level: current_level + 1,
                            },
                            Listing::File { name, size } => FileTree::File {
                                name: name.clone(),
                                size: *size,
                                level: current_level + 1,
                            },
                        };
                        current_node.borrow_mut().add_child(child);
                    }
                }
            }
        }
        Rc::try_unwrap(root_node)
            .unwrap()
            .replace(FileTree::new_root())
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let commands = parse_commands(&contents);
    let tree = FileTree::from_commands(&commands);
    let directories = get_all_directory_sizes(&tree);
    let total_size = directories
        .iter()
        .map(|(_, size)| *size)
        .filter(|s| s <= &100000)
        .sum::<u64>();
    format!("{}", total_size)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let commands = parse_commands(&contents);
    let tree = FileTree::from_commands(&commands);
    let directories = get_all_directory_sizes(&tree);

    let total_space = 70000000;
    let required_free_space = 30000000;
    let current_free_space = total_space - directories[0].1;
    let space_to_free = required_free_space - current_free_space;

    let directory_to_delete_size = directories
        .iter()
        .map(|(_, size)| *size)
        .filter(|s| s >= &space_to_free)
        .min()
        .unwrap();

    format!("{}", directory_to_delete_size)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input07").expect("Failed to open input file")
}

fn parse_commands(contents: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut current_command = None;
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        if line.starts_with('$') {
            if current_command.is_some() {
                let command = current_command.take();
                commands.push(command.unwrap());
            }
            // Command
            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts[1] == "cd" {
                // Change directory
                assert_eq!(parts.len(), 3);
                current_command = match parts[2] {
                    "/" => Some(Command::ChangeDirectory(DirectoryInput::Home)),
                    ".." => Some(Command::ChangeDirectory(DirectoryInput::UpOne)),
                    directory => Some(Command::ChangeDirectory(DirectoryInput::Directory(
                        directory.to_string(),
                    ))),
                };
            } else if parts[1] == "ls" {
                // List files
                assert_eq!(parts.len(), 2);
                current_command = Some(Command::List(Vec::new()));
            } else {
                panic!("Invalid command: {}", parts[0]);
            }
        } else {
            // Listing output
            let parts = line.split(' ').collect::<Vec<&str>>();
            assert_eq!(parts.len(), 2);
            if parts[0] == "dir" {
                // Directory
                let command = current_command.as_mut().unwrap();
                match command {
                    Command::List(ref mut listings) => {
                        listings.push(Listing::Dir(parts[1].to_string()));
                    }
                    _ => panic!(),
                };
            } else {
                // File
                let command = current_command.as_mut().unwrap();
                match command {
                    Command::List(ref mut listings) => {
                        listings.push(Listing::File {
                            size: parts[0].parse::<u64>().expect("Failed to parse file size"),
                            name: parts[1].to_string(),
                        });
                    }
                    _ => panic!(),
                };
            }
        }
    }
    if current_command.is_some() {
        let command = current_command.take();
        commands.push(command.unwrap());
    }
    commands
}

fn get_directory_size(tree: &FileTree) -> u64 {
    assert!(matches!(*tree, FileTree::Directory { .. }));
    let mut directory_size = 0;
    for child in tree.children() {
        match &*child.borrow() {
            FileTree::Directory { .. } => {
                directory_size += get_directory_size(&child.borrow());
            }
            FileTree::File { size, .. } => {
                directory_size += size;
            }
        }
    }
    directory_size
}

fn get_all_directory_sizes(tree: &FileTree) -> Vec<(String, u64)> {
    let mut directories = Vec::new();
    match tree {
        FileTree::Directory { name, .. } => {
            let size = get_directory_size(tree);
            directories.push((name.clone(), size));
        }
        _ => panic!(),
    };
    for child in tree.children() {
        if let FileTree::Directory { .. } = &*child.borrow() {
            directories.extend(get_all_directory_sizes(&child.borrow()));
        }
    }
    directories
}

#[test]
fn test_parse_commands() {
    let contents = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
    let commands = parse_commands(&contents);
    assert_eq!(commands.len(), 10);

    assert_eq!(commands[0], Command::ChangeDirectory(DirectoryInput::Home));
    assert_eq!(
        commands[1],
        Command::List(vec![
            Listing::Dir("a".to_string()),
            Listing::File {
                name: "b.txt".to_string(),
                size: 14848514
            },
            Listing::File {
                name: "c.dat".to_string(),
                size: 8504156
            },
            Listing::Dir("d".to_string()),
        ])
    );
    assert_eq!(
        commands[2],
        Command::ChangeDirectory(DirectoryInput::Directory("a".to_string()))
    );
    assert_eq!(
        commands[3],
        Command::List(vec![
            Listing::Dir("e".to_string()),
            Listing::File {
                name: "f".to_string(),
                size: 29116
            },
            Listing::File {
                name: "g".to_string(),
                size: 2557
            },
            Listing::File {
                name: "h.lst".to_string(),
                size: 62596
            },
        ])
    );
    assert_eq!(
        commands[4],
        Command::ChangeDirectory(DirectoryInput::Directory("e".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::List(vec![Listing::File {
            name: "i".to_string(),
            size: 584,
        }])
    );
    assert_eq!(commands[6], Command::ChangeDirectory(DirectoryInput::UpOne));
    assert_eq!(commands[7], Command::ChangeDirectory(DirectoryInput::UpOne));
    assert_eq!(
        commands[8],
        Command::ChangeDirectory(DirectoryInput::Directory("d".to_string()))
    );
    assert_eq!(
        commands[9],
        Command::List(vec![
            Listing::File {
                name: "j".to_string(),
                size: 4060174,
            },
            Listing::File {
                name: "d.log".to_string(),
                size: 8033020,
            },
            Listing::File {
                name: "d.ext".to_string(),
                size: 5626152,
            },
            Listing::File {
                name: "k".to_string(),
                size: 7214296,
            }
        ])
    );
}

#[test]
fn test_construct_file_tree() {
    let contents = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
    let commands = parse_commands(&contents);
    let tree = FileTree::from_commands(&commands);
    let result = tree.to_string();
    assert_eq!(result, "- / (dir)\n  - a (dir)\n    - e (dir)\n      - i (file, size=584)\n    - f (file, size=29116)\n    - g (file, size=2557)\n    - h.lst (file, size=62596)\n  - b.txt (file, size=14848514)\n  - c.dat (file, size=8504156)\n  - d (dir)\n    - j (file, size=4060174)\n    - d.log (file, size=8033020)\n    - d.ext (file, size=5626152)\n    - k (file, size=7214296)\n");
}

#[test]
fn test_get_directory_size() {
    let contents = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
    let commands = parse_commands(&contents);
    let tree = FileTree::from_commands(&commands);
    let root_size = get_directory_size(&tree);
    assert_eq!(root_size, 48381165);
}

#[test]
fn test_get_all_directory_sizes() {
    let contents = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
    let commands = parse_commands(&contents);
    let tree = FileTree::from_commands(&commands);
    let directories = get_all_directory_sizes(&tree);
    dbg!(&directories);
    assert_eq!(directories.len(), 4);
    assert_eq!(directories[0], ("/".to_string(), 48381165));
    assert_eq!(directories[1], ("a".to_string(), 94853));
    assert_eq!(directories[2], ("e".to_string(), 584));
    assert_eq!(directories[3], ("d".to_string(), 24933642));
}
