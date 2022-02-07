extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("image-processor")
        .version("1.0")
        .author("Vaibhav Singh <0vaibhavsingh0@gmail.com>")
        .about("Does some image processing")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to use")
            .required(true)
            .index(2))
        .arg(Arg::with_name("blur")
            .long("blur")
            .value_name("BLUR_AMOUNT")
            .help("Sets Blur to the image. eg: --blur=4.0")
            .takes_value(true))
        .arg(Arg::with_name("brighten")
            .long("brighten")
            .value_name("BRIGHTEN_AMOUNT")
            .help("Sets Brightness to the image. +ive brightens and -ive darkens. eg: --brighten=4")
            .takes_value(true))
        .arg(Arg::with_name("crop")
            .long("crop")
            .value_name("CROP_VALUE")
            .help("Crops the image. (x,y,width,height) eg: --crop=4,3,5,6")
            .takes_value(true))
        .arg(Arg::with_name("rotate")
            .long("rotate")
            .value_name("ROTATE_VALUE")
            .help("Rotates the image. (90, 180, 270) eg: --rotate=90")
            .takes_value(true))
         .arg(Arg::with_name("invert")
            .long("invert")
            .help("Inverts colors of the image. eg: --invert"))
         .arg(Arg::with_name("grayscale")
            .long("grayscale")
            .help("Greyscales the image. eg: --grayscale"))
         .arg(Arg::with_name("fractal")
            .long("fractal")
            .help("Fractal the image. eg: --fractal"))
        .arg(Arg::with_name("generate")
            .long("generate")
            .value_name("GEN_VALUE")
            .help("Generate the image. u8 (r,b,g) -> (9,1,2) eg: --generate=9,1,2"))
        .get_matches();

    /*
    // get args
    // TODO: refactor this to use match instead of if let
    */
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    let blur_amount = match matches.value_of("blur") {
        Some(blur_amount) => blur_amount.parse::<f32>().expect("Failed to parse blur amount"),
        None => -1.0,
    };

    let brighten_amount = match matches.value_of("brighten") {
        Some(brighten_amount) => brighten_amount.parse::<i32>().expect("Failed to parse brighten amount"),
        None => 0,
    };

    let crop_value: (u32, u32, u32, u32) = match matches.value_of("crop") {
        Some(crop_value) => {
            let values = crop_value.split(",").map(|x| x.parse::<u32>().expect("Failed to parse crop value")).collect::<Vec<u32>>();
            (values[0], values[1], values[2], values[3])
        },
        None => (0, 0, 0, 0),
    };

    let rotate_value = match matches.value_of("rotate") {
        Some(rotate_value) => {
            let value = rotate_value.parse::<i32>().expect("Failed to parse rotate value");
            if value == 90 || value == 180 || value == 270 {
                value
            } else {
                println!("Invalid rotation value. Valid values are 90, 180, 270");
                std::process::exit(1);
            }
        },
        None => 0,
    };

    let generate_value = match matches.value_of("generate") {
        Some(gen_value) => {
            let values = gen_value.split(",").map(|x| x.parse::<u8>().expect("Failed to parse generate value")).collect::<Vec<u8>>();
            (values[0], values[1], values[2])
        },
        None => (0,0,0),
    };

    /*
    // handle args with value
    // TODO: refactor this to use match instead of if let
    */
    if blur_amount != -1.0 {
        println!("Bluring image {} to {} with {}", input, output, blur_amount);
        blur(input.to_string(), output.to_string(), blur_amount);
    }

    if brighten_amount != 0 {
        println!("Brightening image {} to {} with {}", input, output, brighten_amount);
        brighten(input.to_string(), output.to_string(), brighten_amount);
    }

    if crop_value.0 != 0 && crop_value.1 != 0 && crop_value.2 != 0 && crop_value.3 != 0 {
        println!("Croping image {} to {} with {:?}", input, output, crop_value);
        crop(input.to_string(), output.to_string(), crop_value);
    }

    if generate_value.0 != 0 && generate_value.1 != 0 && generate_value.2 != 0 {
        println!("Generating image {} to {} with {:?}", input, output, generate_value);
        generate(output.to_string(), generate_value);
        return;
    }

    if rotate_value != 0 {
        println!("Rotating image {} to {} with {}", input, output, rotate_value);
        rotate(input.to_string(), output.to_string(), rotate_value);
    }

    if matches.is_present("grayscale") {
        println!("Grayscaling image {} to {}", input, output);
        grayscale(input.to_string(), output.to_string());
    }

    if matches.is_present("invert") {
        println!("Inverting image {} to {}", input, output);
        invert(input.to_string(), output.to_string());
    }

    if matches.is_present("fractal") {
        println!("Fractaling image {}", output);
        fractal(output.to_string());
    }
    
    if matches.is_present("generate") {
        println!("Fractaling image {}", output);
        fractal(output.to_string());
    }

}

fn open_image(filename: String, err_msg: &str) -> image::DynamicImage {
    image::open(filename).expect(err_msg)
}

fn save_image(img: &image::DynamicImage, filename: String, err_msg: &str) {
    img.save(filename).expect(err_msg);
}

fn blur(infile: String, outfile: String, blur_amount: f32) {
    let img = open_image(infile, "Failed to open INFILE.");
    let img2 = img.blur(blur_amount);
    save_image(&img2, outfile, "Failed to save OUTFILE.");
}

fn brighten(infile: String, outfile: String, brighten_amount: i32) {
    let img = open_image(infile, "Failed to open INFILE.");
    let img2 = img.brighten(brighten_amount);
    save_image(&img2, outfile, "Failed to save OUTFILE.");
}

fn crop(infile: String, outfile: String, (x, y, width, height): (u32, u32, u32, u32)) {
    let mut img = open_image(infile, "Failed to open INFILE.");
    let img2 = img.crop(x, y, width, height);
    save_image(&img2, outfile, "Failed to save OUTFILE.");
}

fn rotate(infile: String, outfile: String, rotate_value: i32) {
    let img = open_image(infile, "Failed to open INFILE.");

    if rotate_value == 90 {
        let img2 = img.rotate90();
        save_image(&img2, outfile, "Failed to save OUTFILE.");
    } else if rotate_value == 180 {
        let img2 = img.rotate180();
        save_image(&img2, outfile, "Failed to save OUTFILE.");
    } else if rotate_value == 270 {
        let img2 = img.rotate270();
        save_image(&img2, outfile, "Failed to save OUTFILE.");
    } else {
        println!("Invalid rotation value. Valid values are 90, 180, 270");
        std::process::exit(1);
    }
}

fn invert(infile: String, outfile: String) {
    let mut img = open_image(infile, "Failed to open INFILE.");
    img.invert();
    save_image(&img, outfile, "Failed to save OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = open_image(infile, "Failed to open INFILE.");
    let img2 = img.grayscale();
    save_image(&img2, outfile, "Failed to save OUTFILE.");
}

fn generate(outfile: String, (r,b,g): (u8, u8, u8)) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = r;
        let blue = b;
        let green = g;

        *pixel = image::Rgb([red, green, blue]);
    }
    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
