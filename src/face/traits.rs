pub trait FaceDetector {
    fn detect_faces(&self, Array3: &Mat) -> Result<BoundingBox>;
    fn detect_faces(&self, Array3: &Mat, coordinates: BoundingBox) -> Result<Mat>;
}
