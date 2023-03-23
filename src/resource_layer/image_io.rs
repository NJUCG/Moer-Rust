use image::codecs::hdr::HdrDecoder;
use image::Rgb32FImage;

pub fn load_img(file: &str) -> Result<Rgb32FImage, std::io::Error> {
    let img = if !file.ends_with(".hdr") {
        image::io::Reader::open(file).expect("Open image error!")
            .decode().expect("Decode error!").to_rgb32f()
    } else {
        let file = std::fs::File::open(file)?;
        let reader = std::io::BufReader::new(file);
        let hdr_decoder = HdrDecoder::new(reader).unwrap();
        let w = hdr_decoder.metadata().width;
        let h = hdr_decoder.metadata().height;
        let pixels = hdr_decoder.read_image_hdr().unwrap();
        Rgb32FImage::from_fn(w, h, |u, v| { pixels[(u + v * w) as usize] })
    };
    Ok(img)
}