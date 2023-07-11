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

    let MAX_SPACE = 70000000;
    let NEEDED_SPACE = 30000000;
    let TARGET_SPACE = MAX_SPACE - NEEDED_SPACE;
    let CURR_SPACE = calculate_dir_size(&filetree, 0);
    let CURR_UNUSED_SPACE = MAX_SPACE - CURR_SPACE;
    println!("{} {}", CURR_SPACE, CURR_UNUSED_SPACE);
    let mut sizes: Vec<usize> = vec![];
    for i in 1..filetree.nodes.len() as usize {
        let size = calculate_dir_size(&filetree, i);
        sizes.push(size);
        println!("{:#?}", sizes);
    }
    sizes.sort();
    for size in sizes {
        if (CURR_UNUSED_SPACE + size >= NEEDED_SPACE) {
            println!("{} {}",size, CURR_SPACE - size);
            break;
        }
    }
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
