use bevy::prelude::*;

/// 多个点组成的包围路径
pub struct Path<const N: usize>(pub [Vec2; N]);

/// 有向线段
pub struct DirectedLineSegment {
    /// 起点
    begin: Vec2,
    /// 方向，单位向量
    rotation: Vec2,
    /// 长度
    len: f32,
}

impl<const N: usize> Path<N> {
    pub fn random_segment(&self) -> DirectedLineSegment {
        let i = rand::random::<usize>() % N;
        let begin = self.0[i];
        let end = self.0[(i + 1) % N];
        let diff = end - begin;
        DirectedLineSegment {
            begin,
            rotation: diff.normalize(),
            len: diff.length(),
        }
    }
}

impl DirectedLineSegment {
    pub fn random_point(&self) -> Vec2 {
        self.begin + (rand::random::<f32>() * self.len) * self.rotation
    }

    pub const fn rotation(&self) -> Vec2 {
        self.rotation
    }
}
