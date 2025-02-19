use glam::Mat4;
use std::ops::{Add, Mul, Sub};

pub struct Animation {
    pub name: String,
    pub channels: Vec<Option<NodeChannels>>,
}

#[derive(Default, Clone)]
pub struct NodeChannels {
    pub translation: Option<Channel>,
    pub rotation: Option<Channel>,
    pub scale: Option<Channel>,
}

impl NodeChannels {
    pub fn eval(&self, t: f32) -> Mat4 {
        let mut translation = Mat4::IDENTITY;
        let mut rotation = Mat4::IDENTITY;
        let mut scale = Mat4::IDENTITY;

        if let Some(channel) = &self.translation {
            translation = channel.eval(t);
        }
        if let Some(channel) = &self.rotation {
            rotation = channel.eval(t);
        }
        if let Some(channel) = &self.scale {
            scale = channel.eval(t);
        }

        translation * rotation * scale
    }
}

#[derive(Default, Clone)]
pub enum InterpolationType {
    #[default]
    STEP,
    LINEAR,
    CUBICSPLINE,
}

impl InterpolationType {
    pub fn interpolate<T>(&self, values: &Vec<T>, timings: &Vec<f32>, indexes: (usize, usize), time: f32) -> T
    where
        T: Mul<f32, Output = T> + Add<T, Output = T> + Copy + Sub<T, Output = T>,
    {
        match self {
            InterpolationType::STEP => values[indexes.0],
            InterpolationType::LINEAR => {
                let prev_time = timings[indexes.0];
                let next_time = timings[indexes.1];
                let t = (time - prev_time) / (next_time - prev_time);
                values[indexes.0] + (values[indexes.1] - values[indexes.0]) * t
            }
            InterpolationType::CUBICSPLINE => {
                let prev_time = timings[indexes.0];
                let next_time = timings[indexes.1];
                let delta_time = next_time - prev_time;

                let prev_tangent = values[indexes.0 * 3 + 2] * delta_time;
                let next_tangent = values[indexes.1 * 3 + 0] * delta_time;

                let t = (time - prev_time) / delta_time;

                let prev_point = values[indexes.0 * 3 + 1];
                let next_point = values[indexes.1 * 3 + 1];

                let t2 = t * t;
                let t3 = t2 * t;

                let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
                let h10 = t3 - 2.0 * t2 + t;
                let h01 = -2.0 * t3 + 3.0 * t2;
                let h11 = t3 - t2;

                prev_point * h00 + prev_tangent * h10 + next_point * h01 + next_tangent * h11
            }
        }
    }
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

impl Channel {
    pub fn eval(&self, t: f32) -> Mat4 {
        match &self.values {
            ChannelType::Translation(translation) => {
                let indexes = self.get_indexes(t);
                Mat4::from_translation(self.interpolation.interpolate(translation, &self.times, indexes, t))
            }
            ChannelType::Rotation(rotation) => {
                let indexes = self.get_indexes(t);
                Mat4::from_quat(self.interpolation.interpolate(rotation, &self.times, indexes, t))
            }
            ChannelType::Scale(scale) => {
                let indexes = self.get_indexes(t);
                Mat4::from_scale(self.interpolation.interpolate(scale, &self.times, indexes, t))
            }
        }
    }

    fn get_indexes(&self, t: f32) -> (usize, usize) {
        let mut prev = 0;
        for i in 0..self.times.len() {
            if self.times[i] >= t {
                return (prev, i);
            }
            prev = i;
        }
        (0, 0)
    }
}
