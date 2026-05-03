use opencv::highgui;
use opencv::prelude::*;
use opencv::videoio;

// https://medium.com/@ekfqlwcjswl/using-opencv-in-rust-videocapture-d9e817f8e97c was a very helpful resource for this
fn main() -> opencv::Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    if !cap.is_opened()? {
        panic!("Failed to open video capture device");
    }

    loop {
        cap.read(&mut frame)?;
        highgui::imshow("frame", &frame)?;

        if highgui::wait_key(10)? == 27 {
            break;
        }
    }

    Ok(())
}
