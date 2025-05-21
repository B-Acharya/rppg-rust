//use crate::face::traits::FaceDetector;
//use crate::rppg::traits::RppgAlgorithm;
//use crate::types::{BoundingBox, Array3};
//use crate::video::traits::VideoBackend;
//
//pub fn run_pipeline(
//    backend: &dyn VideoBackend,
//    detector: &dyn FaceDetector,
//    rppg: &dyn RppgAlgorithm,
//    video_path: &str,
//) -> Result<f64> {
//    let fps = backend.fps(video_path)?;
//    let frames = backend.get_frames(video_path)?;
//
//    let mut rois = vec![];
//    for frame in &frames {
//        let faces = detector.detect_faces(frame)?;
//        if let Some(face) = faces.first() {
//            rois.push(crop_frame(frame, face));
//        }
//    }
//
//    let mut buffer = vec![];
//    let hr = rppg.extract_hr(&rois, &mut buffer, fps, true);
//    println!("Heart Rate: {:.2} bpm", hr);
//    Ok(hr)
//}
//
//fn crop_frame(frame: &Array3, bbox: &BoundingBox) -> Array3 {
//    let view = frame.data.slice(s![
//        bbox.y..bbox.y + bbox.height,
//        bbox.x..bbox.x + bbox.width,
//        ..
//    ]);
//    Array3 {
//        data: view.to_owned(),
//    }
//}
