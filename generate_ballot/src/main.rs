use generate_ballot::candidates::*;
use image::GenericImageView;

fn main() {
    // Open the image specified by the path
    if let Ok(img) = image::open("ballot/ballot.png") {
        // Get dimensions of the image
        let (width, height) = img.dimensions();

        // Total coordinates would be width * height
        let total_coordinates = width * height;

        println!("Image dimensions: {}x{}", width, height);
        println!("Total coordinates: {}", total_coordinates);
        candidate5(img);
    } else {
        println!("Failed to open the image.");
    }
}
