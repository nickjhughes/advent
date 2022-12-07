use std::fs;

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

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let commands = parse_commands(&contents);
    format!("")
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    format!("")
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

#[derive(Debug, PartialEq, Eq)]
enum FileNode {
    Directory {
        name: String,
        parent: Option<Box<&FileNode>>,
        children: Vec<FileNode>,
    },
    File {
        name: String,
        size: u64,
    },
}

fn construct_file_tree(commands: &[Command]) -> FileNode {
    let mut root_node = FileNode::Directory {
        name: "/".to_string(),
        parent: None,
        children: Vec::new(),
    };
    let mut current_node = &mut root_node;
    for command in commands {
        match command {
            Command::ChangeDirectory(directory_input) => match directory_input {
                DirectoryInput::UpOne => match current_node {
                    FileNode::Directory { parent, .. } => {
                        current_node = parent.as_mut().unwrap();
                    }
                    _ => panic!(),
                },
                DirectoryInput::Home => {
                    current_node = &mut root_node;
                }
                DirectoryInput::Directory(name) => match current_node {
                    FileNode::Directory { children, .. } => {
                        let mut directory_child = None;
                        for child in children {
                            match child {
                                FileNode::Directory {
                                    name: child_name, ..
                                } => {
                                    if child_name == name {
                                        directory_child = Some(child);
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if directory_child.is_none() {
                            directory_child = FileNode::Directory {
                                name: name.to_string(),
                                parent: Some(Box::new(current_node)),
                                children: Vec::new(),
                            };
                        }
                        current_node = directory_child.unwrap();
                    }
                    _ => panic!(),
                },
            },
            Command::List(listings) => todo!(),
        }
    }
    root_node
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
    let tree = construct_file_tree(&commands);
}
