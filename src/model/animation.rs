use crate::model::nodes_tree::Node;
use glam::Quat;
use std::ops::{Add, Mul, Sub};

pub struct Animation {
    pub name: String,
    pub channels: Vec<Option<NodeChannels>>,
    duration: f32
}

impl Animation {
    pub fn new(name: String, channels: Vec<Option<NodeChannels>>) -> Self {
        let mut max_duration = 0.0;
        for channel in &channels {
            if let Some(channel) = channel {
                if let Some(translation) = &channel.translation {
                    let duration = *translation.times.last().unwrap();
                    if duration > max_duration {
                        max_duration = duration;
                    }
                }
                if let Some(rotation) = &channel.rotation {
                    let duration = *rotation.times.last().unwrap();
                    if duration > max_duration {
                        max_duration = duration;
                    }
                }
                if let Some(scale) = &channel.scale {
                    let duration = *scale.times.last().unwrap();
                    if duration > max_duration {
                        max_duration = duration;
                    }
                }
            }
        }
        Self {
            name,
            channels,
            duration: max_duration,
        }
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }
}

#[derive(Default, Clone)]
pub struct NodeChannels {
    pub translation: Option<Channel>,
    pub rotation: Option<Channel>,
    pub scale: Option<Channel>,
}

impl NodeChannels {
    pub fn eval(&self, t: f32, node: &mut Node) {
        if let Some(channel) = &self.translation {
            channel.eval(t, node);
        }
        if let Some(channel) = &self.rotation {
            channel.eval(t, node);
        }
        if let Some(channel) = &self.scale {
            channel.eval(t, node);
        }
    }
}

#[derive(Default, Clone, Debug)]
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
                let prev = values[indexes.0];
                let next = values[indexes.1];
                prev * (1.0 - t) + next * t
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

    pub fn s_interpolate(&self, values: &Vec<Quat>, timings: &Vec<f32>, indexes: (usize, usize), time: f32) -> Quat {
        match self {
            InterpolationType::STEP => values[indexes.0],
            InterpolationType::LINEAR => {
                let prev_time = timings[indexes.0];
                let next_time = timings[indexes.1];
                let t = (time - prev_time) / (next_time - prev_time);
                let prev = values[indexes.0];
                let next = values[indexes.1];
                prev.slerp(next, t)
            }
            InterpolationType::CUBICSPLINE => {
                /*let prev_time = timings[indexes.0];
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
                */
                unimplemented!("Need to implement cubic slerp for quaternions");
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
    pub fn eval(&self, t: f32, node: &mut Node) {
        match &self.values {
            ChannelType::Translation(translation) => {
                let indexes = self.get_indexes(t);
                node.translate = self.interpolation.interpolate(translation, &self.times, indexes, t);
            }
            ChannelType::Rotation(rotation) => {
                let indexes = self.get_indexes(t);
                node.rotate = self.interpolation.s_interpolate(rotation, &self.times, indexes, t);
            }
            ChannelType::Scale(scale) => {
                let indexes = self.get_indexes(t);
                node.scale = self.interpolation.interpolate(scale, &self.times, indexes, t);
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
