use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut filetree = FileTree::new();
    let mut cur = 0;

    for line in input.lines().skip(2) {
        if let Some(dirname) = line.strip_prefix("dir ") {
            let index = filetree.add(cur ,dirname.to_string());
            filetree.nodes[cur].children.push(index);
        } else if let Some(newdir) = line.strip_prefix("$ cd "){
            match newdir {
                ".." => cur = filetree.nodes[cur].parent.unwrap(),
                _ => cur = filetree
                    .nodes[cur]
                    .children
                    .iter()
                    .copied()
                    .find(|&child| match &filetree.nodes[child] {
                        DirNode { name, .. } if name == newdir => true,
                        _ => false,
                    })
                .unwrap()
            };
        } else if line == "$ ls" {
        } else if let Some((size, _)) = line.split_once(' '){
            let size: usize = size.parse().unwrap();
            filetree.nodes[cur].size += size;
        }
    }

    let mut result = 0;
    for i in 0..filetree.nodes.len() as usize {
        let size = calculate_dir_size(&filetree, i);
        if size <= 100000 {
            result += size;
        }
    }
    println!("{}", result);
}

fn calculate_dir_size(filetree: &FileTree, dir: usize) -> usize {
    let dir = &filetree.nodes[dir];
    let mut size = 0;
    size += dir.size;
    for &child in &dir.children {
        size += calculate_dir_size(filetree, child);
    }
    size
}

#[derive(Debug)]
struct FileTree {
    nodes: Vec<DirNode>,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            nodes: vec![DirNode {
                           name: "/".to_string(),
                           children: vec![],
                           parent: None,
                           size: 0,
                       }]
        }
    }
    pub fn add(&mut self, cur: usize, dirname: String) -> usize {
        self.nodes.push(DirNode {
            name: dirname,
            children: vec![],
            parent: Some(cur),
            size: 0,
        });
        self.nodes.len() - 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Index(usize);

#[derive(Debug)]
struct DirNode {
    name: String,
    children: Vec<usize>,
    parent: Option<usize>,
    size: usize,
}
