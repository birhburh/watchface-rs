use {
    image::ImageBuffer,
    std::{
        error::Error,
        fs::{self, File},
        io::{BufWriter, ErrorKind},
    },
    watchface_rs::{parse_watch_face_bin, MiBandParams, PreviewParams, Watchface},
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

    let preview = watchface.generate_preview(Some(PreviewParams {
        hours: Some(12),
        minutes: Some(6),
        seconds: Some(34),
        time12h: true,
        am: false,
        month: Some(3),
        day: Some(23),
        weekday: Some(3),

        steps: Some(12882),
        steps_progress: Some(67),
        distance: Some(14.615483),
        pulse: Some(123),
        heart_progress: Some(43),
        calories: Some(3453),
        calories_progress: Some(20),
        pai: Some(156),

        weather: Some(4),
        temperature: Some(26),
        day_temperature: Some(43),
        night_temperature: Some(-10),
        wind: Some(12),

        battery: Some(64),
        do_not_disturb: true,
        bluetooth: false,
        lock: false,
        alarm_hours: Some(6),
        alarm_minutes: Some(0),
        alarm_on: true,

        animation: Some(0),
        ..Default::default()
    }));

    let mut final_image = ImageBuffer::from_pixel(126, 294, image::Rgba([0, 0, 0, 255]));
    for image in preview {
        let path = format!("{output}/{}.png", image.image_index.0);
        let img = image::open(path).unwrap().into_rgba8();
        image::imageops::overlay(&mut final_image, &img, image.x as i64, image.y as i64);
    }
    let path = format!("{output}/preview.png");
    final_image.save(path).expect("Failed to save final image");

    println!("Written to {output}");
    Ok(())
}
