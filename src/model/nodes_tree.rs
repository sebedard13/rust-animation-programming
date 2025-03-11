use glam::Mat4;
use gltf::scene::Transform;
use crate::utils_glam::decompose;

#[derive(Debug, Clone, Default)]
pub struct Node {
    parent: Option<usize>,
    pub(crate) name: String,
    pub translate: glam::Vec3,
    pub rotate: glam::Quat,
    pub scale: glam::Vec3,
}

pub struct NodeTree {
    pub nodes: Vec<Node>,
    joints_index: Vec<usize>,
    inverse_bind_matrices: Vec<Mat4>,
}

impl NodeTree {
    pub fn get_joints(&self) -> Vec<Mat4> {
        let mut joints = vec![Mat4::IDENTITY; self.joints_index.len()];
        
        for (joint_index, node_index) in  self.joints_index.iter().enumerate() {
            let tree_matrix = self.get_global_transform(*node_index);
            let inverse = self.inverse_bind_matrices[joint_index];
            let matrix = tree_matrix * inverse;

            joints[joint_index] = matrix;
        }
        joints
    }

    pub fn get_joints_double_quat(&self) -> Vec<[glam::Quat;2]> {
        let mut joints = vec![[glam::Quat::IDENTITY;2]; self.joints_index.len()];

        let mat_joints = self.get_joints();
        for mat_index in 0..mat_joints.len() {
            let mat = mat_joints[mat_index];
            //let (_,orientation,translation,_,_) = decompose(mat);
            joints[mat_index][0] = glam::Quat::from_mat4(&mat);
            //joints[mat_index][1] = glam::Quat::from_xyzw(translation.x, translation.y, translation.z, 0.0) * orientation * 0.5;
            joints[mat_index][1] = glam::Quat::from_xyzw(mat.w_axis.x, mat.w_axis.y, mat.w_axis.z, 0.0) * joints[mat_index][0] * 0.5;
            
        }
        joints
    }
    
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl NodeTree {
    pub fn get_local_transform(&self, node_index: usize) -> Mat4 {
        Mat4::from_translation(self.nodes[node_index].translate) * Mat4::from_quat(self.nodes[node_index].rotate) * Mat4::from_scale(self.nodes[node_index].scale)
    }

    pub fn get_global_transform(&self, node_index: usize) -> Mat4 {
        let parent = self.nodes[node_index].parent;
        match parent {
            Some(parent_index) => {
                self.get_global_transform(parent_index) * self.get_local_transform(node_index)
            }
            None => self.get_local_transform(node_index),
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];
                        // level, node_index
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
            println!("{}{}: {}", tabs, node_index, self.nodes[node_index].name);
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
                    let parents_len = tree
                        .iter()
                        .find(|(_, index)| *index == parent_index)
                        .unwrap()
                        .0 +1;
                    tree.push((parents_len, node_index));
                    
                    return parents_len;
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
pub fn create_nodes_tree_from_joints(joints: Vec<usize>, nodes: Vec<gltf::Node>, inverse_bind_matrices: Vec<Mat4>) -> NodeTree {
    let mut node_tree = vec![Node::default(); nodes.len()];

    for (node_index, node) in  nodes.iter().enumerate() {
        let transform = node.transform();
        let (t,r,s) = match transform {
            Transform::Matrix { matrix } => unimplemented!("Matrix not implemented"),
            Transform::Decomposed {
                rotation,
                translation,
                scale,
            } => {
                let translation = glam::Vec3::from(translation);
                let rotation = glam::Quat::from_array(rotation);
                let scale = glam::Vec3::from(scale);
                (translation, rotation, scale)
            }
        };
        
        node_tree[node_index].translate = t;
        node_tree[node_index].rotate = r;
        node_tree[node_index].scale = s;


        if let Some(name) = node.name() {
            node_tree[node_index].name = name.to_string();
        }

        let children: Vec<usize> = node.children().map(|child| child.index()).collect();
        for child in children {
            node_tree[child].parent = Some(node_index);
        }
    }

    NodeTree { nodes: node_tree, inverse_bind_matrices, joints_index: joints, }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn test_hiearchical_matrix(){
        let parent = Node {
            parent: None,
            name: "parent".to_string(),
            translate: glam::Vec3::new(1.0, 0.0, 0.0),
            rotate: glam::Quat::IDENTITY,
            scale: glam::Vec3::new(1.0, 1.0, 1.0),
        };
        let child = Node {
            parent: Some(0),
            name: "child".to_string(),
            translate: glam::Vec3::new(1.0, 0.0, 0.0),
            rotate: glam::Quat::IDENTITY,
            scale: glam::Vec3::new(1.0, 1.0, 1.0),
        };
        
        let tree = vec![parent, child];
        
        let node_tree = super::NodeTree { nodes: tree, inverse_bind_matrices: Vec::new(), joints_index: Vec::new(), };
        
        let child_transform = node_tree.get_global_transform(1);
        assert_eq!(child_transform, glam::Mat4::from_translation(glam::Vec3::new(2.0, 0.0, 0.0)));
    }
}