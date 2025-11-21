use aoc2022::Day;

use std::cmp::{max, min};

#[derive(Debug)]
enum Object<'a> {
    Folder(&'a str, Box<Vec<Object<'a>>>),
    File(&'a str, u32)
}

impl<'a> Object<'a> {
    fn name(&self) -> &'a str {
        match self {
            Self::Folder(n, _) => n,
            Self::File(n, _) => n
        }
    }

    fn get_child(&mut self, name: &str) -> Option<&mut Object<'a>> {
        match self {
            Self::Folder(_, ref mut c) => {
                for (i, o) in (*c).iter_mut().enumerate() {
                    if o.name() == name {
                        return Some(&mut (*c)[i]);
                    }
                }
            },
            Self::File(_, _) => panic!("Files have no children")
        }

        None
    }

    fn get_by_pwd(&mut self, pwd: &Vec<&str>) -> &mut Object<'a> {
        let mut curr_obj: &mut Object<'a> = self;
        for d in pwd {
            curr_obj = curr_obj.get_child(d).unwrap();
        }
        curr_obj
    }

    fn size(&self) -> u32 {
        match self {
            Self::File(_, s) => *s,
            Self::Folder(_, c) => (*c).iter().map(|o| o.size()).sum::<u32>()
        }
    }
}

fn total_folder_size_max(tree: &Object, max: u32) -> u32 {
    if let Object::Folder(_, c) = tree {
        return c.iter()
            .filter(|&o| match o { Object::Folder(_, _) => true, Object::File(_, _) => false })
            .map(|o| o.size())
            .filter(|&s| s <= max)
            .sum::<u32>()
            + c.iter()
                .filter(|&o| match o { Object::Folder(_, _) => true, Object::File(_, _) => false })
                .map(|o| total_folder_size_max(o, max))
                .sum::<u32>();
    }

    panic!("called total_folder_size_max with a file");
}

fn min_folder(tree: &Object, minimum: u32) -> u32 {
    if let Object::Folder(_, c) = tree {
        let c1 = c.iter()
            .filter(|&o| match o { Object::Folder(_, _) => true, Object::File(_, _) => false })
            .map(|o| o.size())
            .filter(|&s| s >= minimum)
            .min().unwrap_or(u32::MAX);
        let c2 = c.iter()
            .filter(|&o| match o { Object::Folder(_, _) => true, Object::File(_, _) => false })
            .map(|o| min_folder(o, minimum))
            .min().unwrap_or(u32::MAX);

        return min(c1, c2);
    }

    panic!("called min_folder with a file");
}

#[allow(dead_code)]
fn print_tree(tree: &Object, depth: usize) {
    match tree {
        Object::Folder(n, c) => {
            println!("{}- {} (dir, size={})", "  ".repeat(depth), n, tree.size());
            c.iter().for_each(|o| print_tree(o, depth + 1));
        }, Object::File(n, s) => println!("{}- {} (file, size={})", "  ".repeat(depth), n, s)
    }
}

pub struct Day7 {}
impl Day for Day7 {
    fn day_number(&self) -> u8 {
        7
    }

    fn part1(&self, input: &String) -> String {
        let mut pwd: Vec<&str> = Vec::new();
        let mut tree = Object::Folder("/", Box::new(Vec::new()));

        for line in input.lines() {
            if line.starts_with('$') {
                if &line[2..=3] == "cd" {
                    match &line[5..] {
                        "/" => pwd.clear(),
                        ".." => _ = pwd.pop(),
                        other => pwd.push(other)
                    }
                }

            } else {
                if line.starts_with('d') { // that means it starts with dir
                    match *tree.get_by_pwd(&pwd) {
                        Object::Folder(_, ref mut c) => c.push(Object::Folder(&line[4..], Box::new(Vec::new()))),
                        Object::File(_, _) => panic!("pwd points to file")
                    }
                } else {
                    let split = line.split_once(' ').unwrap();
                    let size = split.0.parse::<u32>().unwrap();
                    let name = split.1;

                    match *tree.get_by_pwd(&pwd) {
                        Object::Folder(_, ref mut c) => c.push(Object::File(name, size)),
                        Object::File(_, _) => panic!("pwd points to file")
                    }
                }
            }
        }

        // print_tree(&tree, 0);

        total_folder_size_max(&tree, 100000).to_string()
    }

    fn part2(&self, input: &String) -> String {
        let mut pwd: Vec<&str> = Vec::new();
        let mut tree = Object::Folder("/", Box::new(Vec::new()));

        for line in input.lines() {
            if line.starts_with('$') {
                if &line[2..=3] == "cd" {
                    match &line[5..] {
                        "/" => pwd.clear(),
                        ".." => _ = pwd.pop(),
                        other => pwd.push(other)
                    }
                }

            } else {
                if line.starts_with('d') { // that means it starts with dir
                    match *tree.get_by_pwd(&pwd) {
                        Object::Folder(_, ref mut c) => c.push(Object::Folder(&line[4..], Box::new(Vec::new()))),
                        Object::File(_, _) => panic!("pwd points to file")
                    }
                } else {
                    let split = line.split_once(' ').unwrap();
                    let size = split.0.parse::<u32>().unwrap();
                    let name = split.1;

                    match *tree.get_by_pwd(&pwd) {
                        Object::Folder(_, ref mut c) => c.push(Object::File(name, size)),
                        Object::File(_, _) => panic!("pwd points to file")
                    }
                }
            }
        }

        let required_space: u32 = max(0, 30000000 - (70000000 - tree.size()));
        min_folder(&tree, required_space).to_string()
    }
}
