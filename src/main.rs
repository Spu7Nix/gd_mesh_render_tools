mod levelstring;
mod triangle;

use levelstring::*;
use triangle::*;

mod vec3;
use vec3::*;

#[derive(Debug, Clone)]
enum Face {
    Quad([Vec3; 4]),
    Tri([Vec3; 3], [Vector; 3], u8), // texture coord, model id
}

const CULLING: bool = true;
use image::io::Reader as ImageReader;

fn main() {
    //let level = Vec::<GDObject>::new();

    /*let triangle = create_triangle([
        Vector::new(0.0, 0.0),
        Vector::new(2.0, 2.0),
        Vector::new(5.0, 0.0),
    ]);

    let levelstring = create_level_string(triangle);

    encrypt_level_string(levelstring);*/

    let cornell_box = tobj::load_obj("C:/Users/spu7n/OneDrive/Bilder/epic_dragon_boi.obj", true);
    let (models, materials) = cornell_box.unwrap();
    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    let texture = ImageReader::open("C:/Users/spu7n/OneDrive/Bilder/bake1010.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let (tex_w, tex_h) = texture.dimensions();
    println!("w: {}, h: {}", tex_w, tex_h);

    let mut faces = Vec::<Face>::new();
    for model in models {
        let mesh = &model.mesh;
        let m_id = if model.name == "Cube_Cube.001" { 0 } else { 1 };

        let mut next_face = 0;
        for f in 0..mesh.num_face_indices.len() {
            let end = next_face + mesh.num_face_indices[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            match face_indices.len() {
                3 => faces.push(Face::Tri(
                    [
                        Vec3::new(
                            mesh.positions[(face_indices[0] * 3) as usize] as f64,
                            mesh.positions[(face_indices[0] * 3 + 1) as usize] as f64,
                            mesh.positions[(face_indices[0] * 3 + 2) as usize] as f64,
                        ),
                        Vec3::new(
                            mesh.positions[(face_indices[1] * 3) as usize] as f64,
                            mesh.positions[(face_indices[1] * 3 + 1) as usize] as f64,
                            mesh.positions[(face_indices[1] * 3 + 2) as usize] as f64,
                        ),
                        Vec3::new(
                            mesh.positions[(face_indices[2] * 3) as usize] as f64,
                            mesh.positions[(face_indices[2] * 3 + 1) as usize] as f64,
                            mesh.positions[(face_indices[2] * 3 + 2) as usize] as f64,
                        ),
                    ],
                    [
                        Vector::new(
                            mesh.texcoords[(face_indices[0] * 2) as usize],
                            mesh.texcoords[(face_indices[0] * 2 + 1) as usize],
                        ),
                        Vector::new(
                            mesh.texcoords[(face_indices[1] * 2) as usize],
                            mesh.texcoords[(face_indices[1] * 2 + 1) as usize],
                        ),
                        Vector::new(
                            mesh.texcoords[(face_indices[2] * 2) as usize],
                            mesh.texcoords[(face_indices[2] * 2 + 1) as usize],
                        ),
                    ],
                    m_id as u8,
                )),

                a => (), //panic!("only quads and tris are supported, found {}", a),
            }
            next_face = end;
        }
    }
    let mut level = Vec::<GDObject>::new();
    let plane_n = Vec3::new(1.0, 0.0, 0.0);
    if CULLING {
        faces = faces
            .iter()
            .filter(|x| match x {
                Face::Tri([p1, p2, p3], [t1, t2, t3], m_id) => {
                    let v1 = Vec3::between(*p1, *p2);
                    let v2 = Vec3::between(*p1, *p3);
                    let normal = Vec3::new(
                        v1.y * v2.z - v1.z * v2.y,
                        v1.z * v2.x - v1.x * v2.z,
                        v1.x * v2.y - v1.y * v2.x,
                    );
                    //cull backfaces
                    Vec3::dot(plane_n, normal) > -0.05
                }
                _ => unimplemented!(),
            })
            .cloned()
            .collect();
    }

    let plane_start = -8.0;
    let obj_len = 30.0;

    let min_scale = 0.3;
    let perspective_fac = 1.0;

    let subdivisions = 200;
    for p in 0..subdivisions {
        let offset = p as f32 * 1.0;
        let plane_d = plane_start + (p as f64 / subdivisions as f64) * obj_len;

        let mut lines = Vec::<(Vec3, Vec3, f64, Vector, Vector, u8)>::new();
        for f in &faces {
            match f {
                Face::Tri([a, b, c], [t1, t2, t3], m_id) => {
                    if let Some(seg) =
                        triangle_plane_intersection(*a, *b, *c, plane_n, plane_d, (*t1, *t2, *t3))
                    {
                        lines.push((seg.0, seg.1, seg.2, seg.3, seg.4, *m_id))
                    }
                }
                _ => unimplemented!(),
            }
        }

        let top_group = p * 2 + 5;
        let bottom_group = p * 2 + 6;

        let perspective_scale =
            ((subdivisions - p) as f32 / subdivisions as f32) * perspective_fac + min_scale;

        for (m_id, group) in [top_group, bottom_group].iter().enumerate() {
            let mut obj = GDObject::new("901");
            obj.set_pos(Vector::new(1.0, p as f32 * 0.1 + 5.0));

            obj.set_prop(28, &(perspective_scale * -1500.0).to_string());
            obj.set_prop(51, &group.to_string());

            obj.set_prop(10, "25");
            level.push(obj);

            let mut obj2 = GDObject::new("901");
            obj2.set_pos(Vector::new(0.0, p as f32 * 0.1 + 5.0));

            obj2.set_prop(28, &(perspective_scale * 750.0 - offset * 30.0).to_string());
            obj2.set_prop(51, &group.to_string());
            obj2.set_prop(10, "0");
            level.push(obj2);

            let mut obj3 = GDObject::new("901");
            obj3.set_pos(Vector::new(4.0 + m_id as f32 * 2.0, p as f32 * 0.1 + 5.0));

            obj3.set_prop(29, &(perspective_scale.powf(1.2) * 20.0).to_string());
            obj3.set_prop(51, &group.to_string());
            obj3.set_prop(10, "0.5");
            level.push(obj3);

            let mut obj4 = GDObject::new("901");
            obj4.set_pos(Vector::new(8.0 + m_id as f32 * 2.0, p as f32 * 0.1 + 5.0));

            obj4.set_prop(29, &(perspective_scale.powf(1.2) * -20.0).to_string());
            obj4.set_prop(51, &group.to_string());
            obj4.set_prop(10, "0.5");
            level.push(obj4);
        }

        for l in lines {
            let p1 = Vector::new(l.0.y as f32, l.0.z as f32) * perspective_scale;
            let p2 = Vector::new(l.1.y as f32, l.1.z as f32) * perspective_scale;

            let between = Vector::between(p1, p2);
            let len = between.length();
            let mode = 1; //if len < 0.5 { 1 } else { 0 };
            let rad = between.angle() + std::f32::consts::PI / 2.0;

            let angle = rad_to_deg(rad);
            let scale = len * [1.0, 2.0][mode];

            let pos = Vector::middle(p1, p2) + Vector::new(rad.sin(), rad.cos()) * scale * 0.5;

            let hsv = {
                // calculate texture color
                let (tex1, tex2) = (l.3, l.4);
                let samples = 50;
                let between = Vector::between(tex1, tex2);
                let len = Vector::length(&between) / samples as f32;
                let step = between.normalize() * len;
                let mut pixel = [0.0, 0.0, 0.0];
                for i in 0..=samples {
                    let pos = tex1 + step * i as f32;
                    let p = texture.get_pixel(
                        (pos.x * tex_w as f32) as u32,
                        ((1.0 - pos.y) * tex_h as f32) as u32,
                    );

                    pixel[0] += p[0] as f32;
                    pixel[1] += p[1] as f32;
                    pixel[2] += p[2] as f32;
                }
                pixel[0] /= samples as f32;
                pixel[1] /= samples as f32;
                pixel[2] /= samples as f32;

                rgb_to_hsv([pixel[0], pixel[1], pixel[2]])
            };

            if scale > 0.01 {
                //println!("{}", l.2);
                let mut obj = GDObject::new(["1011", "1293"][mode]); //1293
                obj.set_pos(pos + Vector::new(20.0 + offset, 10.0));
                obj.set_prop(6, &(angle as i32).to_string());
                obj.set_prop(32, &(scale).to_string());

                obj.set_prop(20, &(p).to_string());
                obj.set_prop(25, &(20 - p).to_string());
                obj.set_prop(24, &(if plane_d < 0.0 { 5 } else { 1 }).to_string());

                obj.set_prop(
                    57,
                    &(if l.5 == 0 { top_group } else { bottom_group }).to_string(),
                );
                obj.set_prop(21, "1");
                obj.set_prop(41, "1");
                obj.set_prop(
                    43,
                    &format!("{:.3}a{:.3}a{:.3}a0a0", hsv[0], hsv[1], hsv[2]),
                );

                level.push(obj);
            }
        }
    }

    println!("objects: {}", level.len());

    let levelstring = create_level_string(level);

    encrypt_level_string(levelstring);
}

fn dist_from_plane(p: Vec3, plane_n: Vec3, plane_d: f64) -> f64 {
    Vec3::dot(plane_n, p) + plane_d
}
const EPS: f64 = 0.00001;

fn get_segment_plane_intersection(
    p1: Vec3,
    p2: Vec3,
    plane_n: Vec3,
    plane_d: f64,
    t1: Vector,
    t2: Vector,
) -> Option<(Vec3, Vector)> {
    let d1 = dist_from_plane(p1, plane_n, plane_d);
    let d2 = dist_from_plane(p2, plane_n, plane_d);

    if d1 * d2 > EPS {
        return None;
    } // points on the same side of plane

    let t = d1 / (d1 - d2); // 'time' of intersection point on the segment
    let point_out = p1 + (p2 - p1) * t;

    let dist_0 = Vec3::length(Vec3::between(p2, p1));
    let dist_1 = Vec3::length(Vec3::between(p1, point_out)) / dist_0;

    let tex = t1 + Vector::between(t1, t2) * (dist_1 as f32);

    Some((point_out, tex))
}

fn triangle_plane_intersection(
    tri_a: Vec3,
    tri_b: Vec3,
    tri_c: Vec3,
    plane_n: Vec3,
    plane_d: f64,
    (t1, t2, t3): (Vector, Vector, Vector),
) -> Option<(Vec3, Vec3, f64, Vector, Vector)> {
    let ab = get_segment_plane_intersection(tri_a, tri_b, plane_n, plane_d, t1, t2);
    let bc = get_segment_plane_intersection(tri_b, tri_c, plane_n, plane_d, t2, t3);
    let ca = get_segment_plane_intersection(tri_c, tri_a, plane_n, plane_d, t3, t1);

    let seg = match (ab, bc, ca) {
        (Some((p2, tex2)), Some((p1, tex1)), None) => (p1, p2, tex1, tex2),

        (Some((p2, tex2)), None, Some((p1, tex1))) => (p1, p2, tex1, tex2),

        (None, Some((p1, tex1)), Some((p2, tex2))) => (p1, p2, tex1, tex2),

        _ => return None,
    };

    //calculate triangle normal
    let v1 = Vec3::between(tri_a, tri_b);
    let v2 = Vec3::between(tri_a, tri_c);
    let tri_normal_2d = Vector::new(
        (v1.z * v2.x - v1.x * v2.z) as f32,
        (v1.x * v2.y - v1.y * v2.x) as f32,
    )
    .normalize();

    let tri_normal = Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
    .normalize();

    let mut light = Vec3::dot(tri_normal, Vec3::new(0.5, 0.3, 1.0).normalize());
    if light < 0.0 {
        light = 0.0;
    }

    let seg_between = Vector::between(
        Vector::new(seg.0.y as f32, seg.0.z as f32),
        Vector::new(seg.1.y as f32, seg.1.z as f32),
    );
    let seg_normal = Vector::new(-seg_between.y, seg_between.x).normalize();

    let dot = Vector::dot(seg_normal, tri_normal_2d);

    if dot > 0.0 {
        Some((seg.0, seg.1, light, seg.2, seg.3))
    } else {
        Some((seg.1, seg.0, light, seg.3, seg.2))
    }
}
fn rgb_to_hsv(rgb: [f32; 3]) -> [f32; 3] {
    let (max, min, sep, coeff) = {
        let (max, min, sep, coeff) = if rgb[0] > rgb[1] {
            (rgb[0], rgb[1], rgb[1] - rgb[2], 0.0)
        } else {
            (rgb[1], rgb[0], rgb[2] - rgb[0], 2.0)
        };
        if rgb[2] > max {
            (rgb[2], min, rgb[0] - rgb[1], 4.0)
        } else {
            let min_val = if rgb[2] < min { rgb[2] } else { min };
            (max, min_val, sep, coeff)
        }
    };

    let mut h = 0.0;
    let mut s = 0.0;
    let v = max / 255.0;

    if (max - min).abs() > 0.01 {
        let d = max - min;
        s = d / max;
        h = ((sep / d) + coeff) * 60.0 - 180.0;
    };

    [h, s, v]
}
