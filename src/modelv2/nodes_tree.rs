

pub struct Node{
    parent: Option<usize>,

    translate: glam::Vec3,
    rotate: glam::Quat,
    scale: glam::Vec3,
}

pub struct NodeTree{
    nodes: Vec<Node>,
}

impl NodeTree{
    pub fn new() -> Self{
        Self{
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, parent: Option<usize>, translate: glam::Vec3, rotate: glam::Quat, scale: glam::Vec3){
        self.nodes.push(Node{
            parent,
            translate,
            rotate,
            scale,
        });
    }

    pub fn get_local_transform(&self, node_index: usize) -> glam::Mat4{
        let node = &self.nodes[node_index];
        let translate = glam::Mat4::from_translation(node.translate);
        let rotate = glam::Mat4::from_quat(node.rotate);
        let scale = glam::Mat4::from_scale(node.scale);

        translate * rotate * scale
    }

    pub fn get_global_transform(&self, node_index: usize) -> glam::Mat4{
        let mut transform = glam::Mat4::IDENTITY;
        let mut current_node = node_index;
        loop{
            // Maybe work or should be recusive to have correct matrix multiplication order
            transform = self.get_local_transform(current_node) * transform;
            match self.nodes[current_node].parent{
                Some(parent_index) => current_node = parent_index,
                None => break,
            }
        }
        transform
    }
}