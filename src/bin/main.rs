use {
    std::{
        error::Error,
        fs::{self, File},
        io::{BufWriter, ErrorKind},
    },
    watchface_rs::{parse_watch_face_bin, MiBandParams, Watchface},
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("no path given");
    let path = std::path::PathBuf::from(path);
    let output = format!(
        "{}_rs_extracted",
        path.file_stem().unwrap().to_str().unwrap()
    );

    println!("Reading {}", path.to_str().unwrap());
    let bytes = fs::read(&path).expect("no file found");
    let watchface: Watchface<MiBandParams> = parse_watch_face_bin(&mut &bytes[..]).unwrap();
    let res = serde_json::to_string_pretty(&watchface.parameters).unwrap();

    // TODO: Probably better to show error to not remove existing, probably modified, extracted watchface folder
    if let Err(e) = fs::create_dir(&output) {
        match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => return Err(e.into()),
        }
    };

    fs::write(format!("{output}/watchface.json"), res).expect("cannot write watchface.json");

    for (i, image) in watchface.images.iter().enumerate() {
        let file = File::create(format!("{output}/{i}.png")).unwrap();
        let w = &mut BufWriter::new(file);
        let mut enc = png::Encoder::new(w, image.width as u32, image.height as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        enc.set_source_chromaticities(png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        ));
        let mut writer = enc.write_header().unwrap();
        writer.write_image_data(&image.pixels).unwrap();
    }

    println!("Written to {output}");
    Ok(())
}
