pub struct Animation {
    pub name: String,
    pub channels: Vec<NodeChannels>,
}

#[derive(Default, Clone)]
pub struct NodeChannels {
    translation: Option<Channel>,
    rotation: Option<Channel>,
    scale: Option<Channel>,
}

#[derive(Default, Clone)]
enum InterpolationType {
    #[default]
    STEP,
    LINEAR,
    CUBICSPLINE,
}

#[derive(Default, Clone)]
pub enum ChannelType {
    #[default]
    Translation(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Scale(Vec<glam::Vec3>),
}

#[derive(Default, Clone)]
struct Channel {
    pub interpolation: InterpolationType,
    pub times: Vec<f32>,
    pub values: ChannelType,
}
