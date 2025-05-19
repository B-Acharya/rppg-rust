use ndarray::Array3;

pub struct BoundingBox {
    pub center_x: usize,
    pub center_y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Frame {
    pub data: Array3<f32>,
}
