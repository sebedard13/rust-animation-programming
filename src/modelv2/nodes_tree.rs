use gltf::scene::Transform;

#[derive(Debug, Clone, Default)]
pub struct Node {
    parent: Option<usize>,
    name: String,
    transform: glam::Mat4,
}

pub struct NodeTree {
    nodes: Vec<Node>,
}


impl NodeTree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn get_local_transform(&self, node_index: usize) -> glam::Mat4 {
        self.nodes[node_index].transform
    }

    pub fn get_global_transform(&self, node_index: usize) -> glam::Mat4 {
        let mut transform = glam::Mat4::IDENTITY;
        let mut current_node = node_index;
        loop {
            // Maybe work or should be recusive to have correct matrix multiplication order
            transform = self.get_local_transform(current_node) * transform;
            match self.nodes[current_node].parent {
                Some(parent_index) => current_node = parent_index,
                None => break,
            }
        }
        transform
    }

    pub fn print(&self) {
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];
        let mut tree: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();

        let mut root = 0;
        loop {
            self.print_info_rec(root, &mut visited, &mut tree);

            match find_a_false(&visited) {
                Some(index) => {
                    root = index;
                }
                None => break,
            }
        }


        //Sort by parent level or else children could brother could be not near

        for (level, node_index) in tree {
            let tabs = "-".repeat(level);
            println!("{}: {}", tabs, self.nodes[node_index].name);
        }

    }

    fn print_info_rec(
        &self,
        node_index: usize,
        visited: &mut Vec<bool>,
        tree: &mut Vec<(usize, usize)>,
    ) -> usize {
        visited[node_index] = true;
        let parent = self.nodes[node_index].parent;

        match parent {
            Some(parent_index) => {

                if visited[parent_index] {
                    return tree.iter().find(|(_, index)| *index == parent_index).unwrap().0;
                }

                let parents_len = self.print_info_rec(parent_index, visited, tree) + 1;

                tree.push((parents_len, node_index));
                parents_len
            }
            None => {
                tree.push((0, node_index));
                0
            }
        }
    }
}

fn find_a_false(visited: &Vec<bool>) -> Option<usize> {
    for (i, bool) in visited.iter().enumerate() {
        if !bool {
            return Some(i);
        }
    }
    None
}
pub fn create_nodes_tree_from_joints(joints: &Vec<usize>, nodes: Vec<gltf::Node>) -> NodeTree {
    let mut node_tree = vec![Node::default(); joints.len()];

    for joint in joints {
        let node = &nodes[*joint];
        let transform = node.transform();
        let mat4 = match transform {
            Transform::Matrix { matrix } => glam::Mat4::from_cols_array_2d(&matrix),
            Transform::Decomposed {
                rotation,
                translation,
                scale,
            } => {
                let translation = glam::Vec3::from(translation);
                let rotation = glam::Quat::from_vec4(glam::Vec4::from(rotation));
                let scale = glam::Vec3::from(scale);
                glam::Mat4::from_scale_rotation_translation(scale, rotation, translation)
            }
        };

        node_tree[*joint].transform = mat4;

        if let Some(name) = node.name() {
            node_tree[*joint].name = name.to_string();
        }

        let children: Vec<usize> = node.children().map(|child| child.index()).collect();
        for child in children {
            node_tree[child].parent = Some(*joint);
        }
    }

    NodeTree { nodes: node_tree }
}
