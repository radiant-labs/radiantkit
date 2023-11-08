pub fn load_image(
    path: String,
    callback: impl FnOnce(Result<epaint::ColorImage, String>) + 'static + Send,
) {
    ehttp::fetch(ehttp::Request::get(path.clone()), move |response| {
        match response {
            Ok(response) => {
                // println!("Response: {:?}", response);
                // let mime_type = response.content_type().map(|v| v.to_owned());

                let result =
                    match image::load_from_memory(&response.bytes).map_err(|err| err.to_string()) {
                        Ok(image) => {
                            let size = [image.width() as _, image.height() as _];
                            let image_buffer = image.to_rgba8();
                            let pixels = image_buffer.as_flat_samples();
                            Ok(epaint::ColorImage::from_rgba_unmultiplied(
                                size,
                                pixels.as_slice(),
                            ))
                        }
                        Err(err) => {
                            log::error!("Failed to load {path:?}: {err}");
                            Err(err)
                        }
                    };
                callback(result);
            }
            Err(err) => {
                log::error!("Failed to load {path:?}: {err}");
            }
        };
    });
}
