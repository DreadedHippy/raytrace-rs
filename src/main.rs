use crate::{color::{write_color, Color}, ray::Ray, vec3::{Point3, Vec3}};


pub mod vec3;
pub mod color;
pub mod ray;
pub mod camera;

// const ASPECT_RATIO: f64 = 16.0/9.0;
// const IMAGE_WIDTH: i32 = 400;

// // Ensure image height is at least 1
// const IMAGE_HEIGHT: i32 = if ((IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32) < 1 {1} else {(IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32};

// // Viewport widths less than one are ok since they are real valued.
// const VIEWPORT_HEIGHT: f64 = 2.0;
// const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64/IMAGE_HEIGHT as f64);

fn ray_color(r: &Ray) -> Color {
    // Scale ray direction to unit vector;
    let unit_direction = Vec3::unit_vector(&r.direction()); // now -1.0 <= y <= 1.0
    let a = 0.5 * (unit_direction.y() + 1.0); // a is the equivalent of y in the interval 0.0, 1.0, i,e 0.0 <= a <= 1.0
    (1.0-a)*Color::from_xyz(1.0, 1.0, 1.0) + a*Color::from_xyz(0.5, 0.7, 1.0)
}


fn main() {
    // println!("Hello, world!");

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = ((image_width as f64/ aspect_ratio) as i32).max(1);

    // Camera
    let focal_length = 1.0; // distance from viewport to camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    let camera_center = Point3::from_xyz(0.0, 0.0, 0.0);

    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from_xyz(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from_xyz(0.0, -viewport_height, 0.0);

    
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    // Remember: Our pixel grid will be inset from the viewport edges by half the pixel-to-pixel distance
    let pixel_delta_u = viewport_u/image_width as f64;
    let pixel_delta_v = viewport_v/image_height as f64;

    // Calculate the location of the upper left pixel.
    // Remember viewport-to-camera distance is focal_length

    let viewport_upper_left =
        camera_center // start from camera center
        - Vec3::from_xyz(0.0, 0.0, focal_length) // Remember viewport-to-camera distance is focal_length in the negative-z direction, at this point, we are at viewport center
        - viewport_u/2.0 // The viewport center is halfway between left and right side so with this we go to the left
        - viewport_v/2.0; // The viewport center is also half-way between top and bottom so with this we go to top-left

    // Now to calculate position of upper-left pixel...
    // We know that the pixel grid is inset from the viewport by half the pixel-delta both top and left...
    // which gives us
    let pixel00_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

    let img_width = 256;
    let img_height = 256;

    // Render
    println!("P3\n{} {}\n255", img_width, img_height);

    for j in 0..img_height {
        eprintln!("\rScanlines remaining: {}", (img_height - j));
        for i in 0..img_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from_values(&pixel_center, &ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
        eprint!("\rDone         ");
    }



}
