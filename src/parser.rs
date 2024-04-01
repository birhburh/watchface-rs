use {
    crate::common::*, // TODO: not use star
    std::{
        collections::{hash_map::Entry, HashMap}, mem::size_of
    },
    winnow::{
        binary::{be_u16, le_f32, le_u16, le_u32, u8},
        stream::{Located, Location, Stream as _},
        token, PResult, Parser,
    },
};

pub type Stream<'i> = Located<&'i [u8]>;

pub fn variable_width_value_parser(i: &mut Stream) -> PResult<(i64, usize)> {
    let mut value = 0i64;
    let bytes = token::take_till(0..10, |b| b & 0x80 != 0x80).parse_next(i)?;
    let last = u8.parse_next(i)?;
    let mut i = 0;
    for b in [bytes, &[last]].concat() {
        value |= (b as i64 & 0x7f) << (i * 7);
        i += 1;
    }

    Ok((value, i))
}

pub fn write_variable_width_value(value: i64) -> Vec<u8> {
    let mut result = vec![];
    let mut value_big_int = value as u64;

    for _ in 0..10 {
        // Read lower 7 bits of value
        let byte = (value_big_int & 0x7F) as u8;

        value_big_int >>= 7;

        // If no data left after this byte, we can stop
        if value_big_int == 0 {
            result.push(byte);
            break;
        }
        // Add byte with flag meaning the data continues in the next byte
        result.push(byte | 0x80)
    }

    result
}

pub fn image_parse(i: &mut Stream) -> PResult<Image> {
    let signature = le_u16.parse_next(i)?;
    if signature != 0x4D42 {
        panic!("Invalid image signature: {}", signature);
    }

    // read header
    let pixel_format = le_u16.parse_next(i)?;

    if pixel_format == 0x65 {
        todo!();
        // return parseCompressedImage(dataBuffer)
    } else if pixel_format == 0xFFFF {
        todo!();
        // return parse32BitImage(dataBuffer)
    }

    let width = le_u16.parse_next(i)?;
    let height = le_u16.parse_next(i)?;
    let row_size = le_u16.parse_next(i)?;
    let bits_per_pixel = le_u16.parse_next(i)?;
    let palette_colors_count = le_u16.parse_next(i)?;
    let transparent_palette_color = le_u16.parse_next(i)?;

    if !([16, 24, 32].contains(&bits_per_pixel)
        && palette_colors_count == 0
        && [0x08, 0x13, 0x1B, 0x1C, 0x10, 0x09].contains(&pixel_format)
        || [1, 2, 4, 8].contains(&bits_per_pixel)
            && palette_colors_count > 0
            && pixel_format == 0x64)
    {
        panic!("Unsuported pixel format/color depth/Palette (should add support) {pixel_format} {bits_per_pixel} {palette_colors_count}")
    }

    if ((bits_per_pixel * width) as f32 / 8.).ceil() as u16 != row_size {
        panic!("Row size is not as expected (Padding ?)")
    }

    let mut palette = vec![];

    if palette_colors_count > 0 {
        // Read palette
        for color_number in 0..palette_colors_count {
            let color = (
                u8.parse_next(i)?,
                u8.parse_next(i)?,
                u8.parse_next(i)?,
                if (transparent_palette_color != 0)
                    && (color_number == transparent_palette_color - 1)
                {
                    0xFF
                } else {
                    0x00
                },
            );
            u8.parse_next(i)?;
            palette.push(color);
        }
    }

    let mut prev_byte = -1;
    let mut val = 0;
    // Read pixel data
    let mut pixels = vec![0; 4usize * width as usize * height as usize];
    for y in 0..height {
        for x in 0..width {
            // read pixel color info
            let red;
            let green;
            let blue;
            let mut alpha = 0x00;
            if palette_colors_count != 0 {
                let color_id;
                if bits_per_pixel < 8 {
                    let pixels_per_byte = 8 / bits_per_pixel;

                    let new_byte = ((x / pixels_per_byte) as f32).floor() as i32;
                    let byte = if new_byte != prev_byte {
                        prev_byte = new_byte;
                        val = u8.parse_next(i)?;
                        val
                    } else {
                        val
                    };
                    let bit_mask = (1 << bits_per_pixel) - 1;
                    let bit_position = 8 - ((x % pixels_per_byte) + 1) * bits_per_pixel;
                    color_id = (byte >> bit_position) & bit_mask;
                } else {
                    color_id = u8.parse_next(i)?;
                }
                (red, green, blue, alpha) = palette[color_id as usize];
            } else {
                let byte_per_pixel = bits_per_pixel / 8;

                if byte_per_pixel == 4 {
                    red = u8.parse_next(i)?;
                    green = u8.parse_next(i)?;
                    blue = u8.parse_next(i)?;
                    alpha = u8.parse_next(i)?;
                } else {
                    let rgba;
                    if byte_per_pixel == 3 {
                        // 24 bits is 16 bit color data (big endian) with 8 bit alpha
                        alpha = u8.parse_next(i)?;
                        rgba = be_u16.parse_next(i)?;
                    } else {
                        // for the 16 bit images, the value is little endian
                        rgba = le_u16.parse_next(i)?;
                    }
                    if pixel_format == 0x13 {
                        // color is 16 bit (4:4:4:4) abgr
                        alpha = ((rgba & 0xF000) >> 8) as u8;
                        blue = ((rgba & 0x0F00) >> 4) as u8;
                        green = (rgba & 0x00F0) as u8;
                        red = ((rgba & 0x000F) << 4) as u8;
                    } else if pixel_format == 0x1C || pixel_format == 0x09 {
                        // color is 16bit (5:6:5) rgb
                        red = ((rgba & 0xF800) >> 8) as u8;
                        green = ((rgba & 0x07E0) >> 3) as u8;
                        blue = ((rgba & 0x001F) << 3) as u8;
                    } else {
                        // color is 16bit (5:6:5) bgr
                        blue = ((rgba & 0xF800) >> 8) as u8;
                        green = ((rgba & 0x07E0) >> 3) as u8;
                        red = ((rgba & 0x001F) << 3) as u8;
                    }
                }
            }

            let pixel_position = (y as usize * width as usize + x as usize) * 4;
            pixels[pixel_position] = red;
            pixels[pixel_position + 1] = green;
            pixels[pixel_position + 2] = blue;
            // Alpha is inverted, 0xFF is transparent
            pixels[pixel_position + 3] = 0xFF - alpha;
        }
    }

    Ok(Image {
        width,
        height,
        bits_per_pixel,
        pixel_format,
        pixels,
    })
}

pub fn params_parser(i: &mut Stream, max_size: usize) -> PResult<Params> {
    let mut prev = i.location();
    let mut bytes_left = max_size;
    let mut params = Params::from(HashMap::new());
    while bytes_left > 0 {
        match param_parser.parse_next(i) {
            Ok(o) => {
                let (key, val) = o;
                match params.entry(key) {
                    Entry::Occupied(mut occupied) => {
                        occupied.get_mut().push(val);
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(vec![val]);
                    }
                }
                bytes_left -= i.location() - prev;
                prev = i.location();
            }
            Err(e) => return Err(e),
        }
    }
    Ok(params)
}

pub fn param_parser(i: &mut Stream) -> PResult<(u8, Param)> {
    // Read parameters info
    let (field_descriptor, _) = variable_width_value_parser(i)?;

    let key = (field_descriptor >> 3) as u8;
    let has_child = field_descriptor & 0x02 == 0x02;

    // From the second byte on is the value
    let value;
    let is_float = field_descriptor & 0x05 == 0x05;
    if is_float {
        value = Param::Float(le_f32.parse_next(i)?);
    } else {
        // variable width value
        let (field_value, _) = variable_width_value_parser(i)?;

        if has_child {
            // When node has Child, field value is size of Child

            let child_size = field_value as usize;
            // Recursive call to read Child data
            let child = params_parser(i, child_size)?;
            value = Param::Child(child);
        } else {
            value = Param::Number(field_value);
        }
    }
    Ok((key, value))
}

pub fn bytes_to_usize(bytes: &[u8]) -> usize {
    let zeros = (0..(size_of::<usize>() - bytes.len()))
        .map(|_| 0u8)
        .collect::<Vec<_>>();
    let bytes = [bytes, &zeros[..]].concat();
    let bytes = bytes[0..size_of::<usize>()].try_into().unwrap();
    usize::from_le_bytes(bytes)
}

pub fn bin_parser<T>(mut i: Located<&[u8]>) -> PResult<Watchface<T>>
where
    T: WatchfaceParams,
    Option<T>: Transform,
{
    let _signature = token::take(4usize).parse_next(&mut i)?;
    let _header = token::take(75usize).parse_next(&mut i)?;
    let _buffer_size = le_u32.parse_next(&mut i)?;
    let info_size = le_u32.parse_next(&mut i)?;
    let parameter_info = params_parser(&mut i, info_size as usize)?;

    // First parameter info contains parameters size and images count
    use Param::*;

    let first_parameter = match &parameter_info.get(&1).unwrap()[0] {
        Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    let mut parameters_size: usize = 0;
    parameters_size.transform(first_parameter.get(&1).unwrap());
    let mut images_count: usize = 0;
    images_count.transform(first_parameter.get(&2).unwrap());

    let mut parameters: Option<T> = None;
    let mut all_params = HashMap::new();
    let params_start = i.checkpoint();

    for (key, value) in parameter_info.iter() {
        if *key == 1 {
            continue;
        }

        i.reset(&params_start);

        let subvalue = match value.get(0).unwrap() {
            Child(child) => child,
            _ => panic!("First param should be child param"), // TODO: use adequate messages in all panics
        };

        let mut offset: usize = 0;
        offset.transform(subvalue.get(&1).unwrap());
        let mut size: usize = 0;
        size.transform(subvalue.get(&2).unwrap());

        i.next_slice(offset);
        let params = params_parser(&mut i, size)?;
        all_params.insert(*key, vec![Param::Child(params)]);
    }

    let params = &vec![Param::Child(all_params)];
    parameters.transform(params);

    i.reset(&params_start);
    i.next_slice(parameters_size);

    let images_info_size = 4 * images_count;
    let images_info = token::take(images_info_size).parse_next(&mut i)?;

    let images_start = i.checkpoint();

    // Load each image
    let mut images = vec![];
    for offset_index in 0..images_count {
        let image_offset = bytes_to_usize(&images_info[offset_index * 4..offset_index * 4 + 4]);
        i.reset(&images_start);
        i.next_slice(image_offset);
        let image = image_parse(&mut i)?;
        images.push(image);
    }

    Ok(Watchface { parameters, images })
}

#[cfg(test)]
mod tests {
    use {super::*, std::collections::HashMap};

    #[test]
    fn parse_keys_and_values() {
        let bytes: Vec<u8> = vec![0x08, 0x04, 0x10, 0x6B];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([
                    (1, vec![Param::Number(0x04)]),
                    (2, vec![Param::Number(0x6B)]),
                ]))
            )
        }
    }

    #[test]
    fn parse_nested_structure() {
        let bytes: Vec<u8> = vec![0x0A, 0x05, 0x08, 0xBC, 0x04, 0x10, 0x6B];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([(
                    1,
                    vec![Param::Child(Params::from(HashMap::from([
                        (1, vec![Param::Number(0x023C)]),
                        (2, vec![Param::Number(0x6B)]),
                    ])))]
                ),]))
            )
        }
    }

    #[test]
    fn parse_lists() {
        let bytes: Vec<u8> = vec![0x08, 0x04, 0x08, 0x7F, 0x10, 0x6B];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([
                    (1, vec![Param::Number(0x04), Param::Number(0x7F)]),
                    (2, vec![Param::Number(0x6B)]),
                ]))
            )
        }
    }

    #[test]
    fn parse_multi_byte_id() {
        let bytes: Vec<u8> = vec![0x80, 0x02, 0x04];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([(32, vec![Param::Number(0x04)]),]))
            )
        }
    }

    #[test]
    fn parse_float_values() {
        let bytes: Vec<u8> = vec![
            0x0A, 0x0A, 0x0D, 0x00, 0x00, 0xA0, 0x3F, 0x3D, 0x00, 0x00, 0xB4, 0x43,
        ];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([(
                    1,
                    vec![Param::Child(Params::from(HashMap::from([
                        (1, vec![Param::Float(1.25)]),
                        (7, vec![Param::Float(360.0)])
                    ])),)]
                ),]))
            )
        }
    }

    #[test]
    fn read_single_byte_value() {
        let bytes: Vec<u8> = vec![0x73];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (0x73, 1),)
        }
    }

    #[test]
    fn read_multi_byte_value() {
        let bytes: Vec<u8> = vec![0xF3, 0x42];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (0x2173, 2),)
        }
    }

    #[test]
    fn read_negative_values() {
        let bytes: Vec<u8> = vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (-13, 10),)
        }
    }

    #[test]
    fn read_32bit_negative_values() {
        let bytes: Vec<u8> = vec![0xF3, 0xFF, 0xFF, 0xFF, 0x0F];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                // TODO: implement 32 bit reading because it is needed for UIHH_BIPU_GTS2MINI format
                // we can live without it for now
                (value as i32, value_size),
                (-13, 5),
            )
        }
    }

    #[test]
    fn read_31_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x04];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value as i32, value_size), (1073741824, 5),)
        }
    }

    #[test]
    fn read_32_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (2147483648, 5),)
        }
    }

    #[test]
    fn read_33_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x10];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (4294967296, 5),)
        }
    }

    #[test]
    fn write_small_value_on_one_byte() {
        assert_eq!(write_variable_width_value(0x73), vec![0x73],)
    }

    #[test]
    fn write_bigger_values_on_multiple_bytes() {
        assert_eq!(write_variable_width_value(0x2173), vec![0xF3, 0x42],)
    }

    #[test]
    fn write_negative_values() {
        assert_eq!(
            write_variable_width_value(-13),
            vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
        )
    }

    // #[test]
    // fn write_32bit_negative_values () {
    //     assert_eq!(
    //         write_variable_width_value(-13, 32),
    //         vec![0xF3, 0xFF, 0xFF, 0xFF, 0x0F],
    //     )
    // }

    #[test]
    fn write_31_bit_value() {
        assert_eq!(
            write_variable_width_value(1073741824),
            vec![0x80, 0x80, 0x80, 0x80, 0x04],
        )
    }

    #[test]
    fn write_32_bit_value() {
        assert_eq!(
            write_variable_width_value(2147483648),
            vec![0x80, 0x80, 0x80, 0x80, 0x08],
        )
    }

    #[test]
    fn write_33_bit_value() {
        assert_eq!(
            write_variable_width_value(4294967296),
            vec![0x80, 0x80, 0x80, 0x80, 0x10],
        )
    }
}
