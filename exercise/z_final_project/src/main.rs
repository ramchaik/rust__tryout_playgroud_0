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

    // **OPTION**
    // Rotate -- see the rotate() function below

    // **OPTION**
    // Invert -- see the invert() function below

    // **OPTION**
    // Grayscale -- see the grayscale() function below

    // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
    //     "fractal" => {
    //         if args.len() != 1 {
    //             print_usage_and_exit();
    //         }
    //         let outfile = args.remove(0);
    //         fractal(outfile);
    //     }
    //
    //     // **OPTION**
    //     // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
    //
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

fn rotate(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
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
