use core::f64;
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
fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    // get (C - Q) and store
    let oc = *center - r.origin();
    let a = r.direction().length_squared();
    let h = Vec3::dot(&r.direction(), &oc);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = h*h - a*c;

    // eprintln!("{:?}", discriminant);

    // return discriminant >= 0.0;

    if discriminant < 0.0 {
        return -1.0
    } else {
        let t1 = (h - f64::sqrt(discriminant))/a;
        return t1
    }

}

// My attempt at checking for ray collision with cube
// Parallel ray epsilon
const E: f64 = 0.000001;
fn hit_cube(center: &Point3, side_length: f64, ray: &Ray) -> bool {
    // A cube in 3d geometric space at origin (0, 0, 0) with side length 2a
    // i.e, sides extend from -a to +a is defined by the inequalities
    // -a <= x <= a, -a <= y <= a, -a <= z <= a
    // starting from origin h, k, l gives us
    // h-a <= x <= h+a, k-a <= y <= k+a, l-a <= z <= l+a
    // let p be a point with coordinates (x, y, z) and C be the cube center (h, k, l),
    // and A be (a, a, a)  then:
    // C-A = (h-a, k-a, l-a),
    // C+A = (h+a, k+a, l+a)
    // by this, the point p must be between C-A and C+A
    // i.e C-A <= p <= C+A at any coordinate
    // because p is a function of t
    // C-A <= Q + t*d <= C+A;
    // taking each inequality:
    // left inequality
    // C-A <= Q + t*d;
    // C-A - Q <= t*d;
    // right inequality
    // Q + t*d <= C+A;
    // t*d <= C+A - Q;
    // hence:
    // (C-A-Q) <= t*d <= (C+A-Q);
    // let's code it up

    let c = *center; // C
    let a = side_length/2.0;
    let a = Vec3::from_xyz(a, a, a); // A
    let q = ray.origin(); // Q
    let v1 = c-a;
    let v2 = c+a;
    let d = Vec3::unit_vector(&ray.direction());
    // if v1 <= t*d <= v2 for any given t
    // let set our t between 0, and infinity i.e f64::MAX

    // check for x

    let mut t_min = 0.0_f64;
    let mut t_max = f64::MAX;

    for (((v1, v2), q), d) in v1.iter().zip(v2.iter()).zip(q.iter()).zip(d.iter()) {
        let mut t1 = (v1 - q)/d;
        let mut t2 = (v2 - q)/d;

        if d.abs() < E {
            if q < v1 || q > v2 {
                return false
            }
        } else {

            if t1 > t2 {(t1, t2) = (t2, t1)}

            t_min = t_min.max(t1);
            t_max = t_max.min(t2);

            if t_min > t_max {return false}
        }

        // eprintln!("{:?}", (t_min, t_max))
    }

    return true;
}


fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::from_xyz(0.0, 0.0, -1.0), 0.5, r);
    // eprintln!("{}", t);

    // if t {
    //     return Color::from_xyz(1.0, 0.0, 0.0)
    // }

    if t > 0.0 {
        // return Color::from_xyz(1.0, 0.0, 0.0);
        // distance from center to hit point, normalized as the normal vector (all components between -1 and 1)
        // each component scaled to (0 <= 1)
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::from_xyz(0.0, 0.0, -1.0)));
        let color = Color::from_xyz(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        return 0.5 * color;
    }

    // eprintln!("NO t");

    // if t.is_nan() {
    //     return Color::from_xyz(1.0, 0.0, 0.0)
    // }

    // if hit_cube(&Point3::from_xyz(0.5, 0.5, -1.0), 0.5, r) {
    //     return Color::from_xyz(1.0, 0.0, 0.0);
    // }

    // Scale ray direction to unit vector;
    let unit_direction = Vec3::unit_vector(&r.direction()); // now -1.0 <= y <= 1.0
    let a = 0.5 * (unit_direction.y() + 1.0); // a is the equivalent of y in the interval 0.0, 1.0, i,e 0.0 <= a <= 1.0
    ((1.0-a)*Color::from_xyz(1.0, 1.0, 1.0)) + (a*Color::from_xyz(0.5, 0.7, 1.0))
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

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from_values(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
        eprint!("\rDone         ");
    }



}
