use rtiaw::{
    camera::Camera,
    object::{Object, ObjectMaterial},
    vector::{Colour, Direction, Point},
};

fn main() {
    let start_time = std::time::Instant::now();

    let scene = make_scene();

    let camera_center = Point::new(13., 2., 3.);
    let camera_forward = camera_center.point_towards(Point::new(0., 0., 0.));
    let camera_left = Direction::new(0., 1., 0.).cross(camera_forward);
    let camera_up = camera_forward.cross(camera_left);

    let camera = Camera::builder()
        .input_position(camera_center, camera_forward, camera_up)
        .input_sensor(1600, 900, 100, 50)
        .input_lens(10., 0.125, 20.)
        .build();
    let image = camera.capture_image(&scene);

    println!(
        "Completed rendering in {:?}. Now saving to `render.ppm`.",
        start_time.elapsed()
    );
    let ppm = rtiaw::ppm::image_to_ppm(&image);
    std::fs::write("render.ppm", ppm.as_bytes()).expect("Expect render saved to file.");
}

fn make_scene() -> Vec<Object> {
    let mut scene = vec![Object::new_sphere(
        Point::new(0., -1000., 0.),
        1000.,
        rtiaw::object::ObjectMaterial::Lambert {
            albedo: Colour::new(0.5, 0.5, 0.5),
        },
    )];

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if center.point_towards(Point::new(4., 0.2, 0.)).length() > 0.9 {
                let material = match rand::random::<f64>() {
                    a if (0.0..0.85).contains(&a) => {
                        let albedo = Colour::new_random() * Colour::new_random();
                        rtiaw::object::ObjectMaterial::Lambert { albedo }
                    }
                    a if (0.85..0.9).contains(&a) => {
                        let albedo = (Colour::new_random() / 2.) + 0.5;
                        let fuzzy_scatter = rand::random::<f64>() / 2.;
                        ObjectMaterial::Metal {
                            albedo,
                            fuzzy_scatter,
                        }
                    }
                    _ => ObjectMaterial::Dialectric {
                        refraction_index: 1.5,
                    },
                };
                scene.push(Object::new_sphere(center, 0.2, material));
            }
        }
    }

    scene.push(Object::new_sphere(
        Point::new(0., 1., 0.),
        1.,
        ObjectMaterial::Dialectric {
            refraction_index: 1.5,
        },
    ));
    scene.push(Object::new_sphere(
        Point::new(-4., 1., 0.),
        1.,
        ObjectMaterial::Lambert {
            albedo: Colour::new(0.4, 0.2, 0.1),
        },
    ));
    scene.push(Object::new_sphere(
        Point::new(4., 1., 0.),
        1.,
        ObjectMaterial::Metal {
            albedo: Colour::new(0.7, 0.6, 0.5),
            fuzzy_scatter: 0.,
        },
    ));

    scene
}
