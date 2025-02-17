pub struct Animation {
    pub name: String,
    pub channels: Vec<NodeChannels>,
}

#[derive(Default, Clone)]
pub struct NodeChannels {
    pub translation: Option<Channel>,
    pub rotation: Option<Channel>,
    pub scale: Option<Channel>,
}

#[derive(Default, Clone)]
pub enum InterpolationType {
    #[default]
    STEP,
    LINEAR,
    CUBICSPLINE,
}

#[derive(Clone)]
pub enum ChannelType {
    Translation(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Scale(Vec<glam::Vec3>),
}

impl Default for ChannelType {
    fn default() -> Self {
        ChannelType::Translation(Vec::new())
    }
}

#[derive(Default, Clone)]
pub struct Channel {
    pub interpolation: InterpolationType,
    pub times: Vec<f32>,
    pub values: ChannelType,
}
