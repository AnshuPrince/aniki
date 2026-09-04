/// Screen OCR for coding-interview context.
///
/// macOS uses Screen Recording + the Vision framework. Other platforms return a
/// clear unsupported message until a Windows OCR path is added.

#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::CStr;
    use std::io::Cursor;

    use cocoa::base::{id, nil};
    use objc::{class, msg_send, sel, sel_impl};
    use xcap::Monitor;

    #[link(name = "Vision", kind = "framework")]
    extern "C" {}

    pub fn recognize_main_display() -> Result<String, String> {
        let monitors = Monitor::all().map_err(|e| e.to_string())?;
        let monitor = monitors
            .into_iter()
            .next()
            .ok_or_else(|| "No display available for OCR".to_string())?;
        let image = monitor.capture_image().map_err(|e| {
            format!(
                "Screen capture failed ({e}). Grant Screen Recording to Aniki in System Settings."
            )
        })?;

        let mut png = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .map_err(|e| e.to_string())?;

        unsafe { vision_recognize_png(&png) }
    }

    unsafe fn vision_recognize_png(png: &[u8]) -> Result<String, String> {
        let data: id = msg_send![
            class!(NSData),
            dataWithBytes: png.as_ptr()
            length: png.len()
        ];
        if data == nil {
            return Err("Failed to wrap screenshot for OCR".into());
        }

        let handler: id = msg_send![class!(VNImageRequestHandler), alloc];
        let handler: id = msg_send![handler, initWithData: data options: nil];
        if handler == nil {
            return Err("Vision image handler failed".into());
        }

        let request: id = msg_send![class!(VNRecognizeTextRequest), new];
        let _: () = msg_send![request, setRecognitionLevel: 0i64]; // accurate
        let requests: id = msg_send![class!(NSArray), arrayWithObject: request];

        let mut error: id = nil;
        let ok: bool = msg_send![handler, performRequests: requests error: &mut error];
        if !ok {
            return Err("Vision text recognition failed".into());
        }

        let results: id = msg_send![request, results];
        let count: usize = msg_send![results, count];
        let mut lines = Vec::new();
        for i in 0..count {
            let obs: id = msg_send![results, objectAtIndex: i];
            let candidates: id = msg_send![obs, topCandidates: 1usize];
            let first: id = msg_send![candidates, firstObject];
            if first == nil {
                continue;
            }
            let nsstring: id = msg_send![first, string];
            let utf8: *const i8 = msg_send![nsstring, UTF8String];
            if utf8.is_null() {
                continue;
            }
            if let Ok(text) = CStr::from_ptr(utf8).to_str() {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    lines.push(trimmed.to_string());
                }
            }
        }

        if lines.is_empty() {
            Ok(String::new())
        } else {
            Ok(lines.join("\n"))
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod macos {
    pub fn recognize_main_display() -> Result<String, String> {
        Err("Screen OCR is implemented on macOS only for now.".into())
    }
}

pub fn capture_screen_ocr() -> Result<String, String> {
    macos::recognize_main_display()
}
