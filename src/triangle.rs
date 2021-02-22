use super::levelstring::GDObject;

// vertex position coordinates are
// in units of full blocks
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, other: Vector) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(v1: Vector, v2: Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn angle(&self) -> f32 {
        let len = self.length();
        (self.x / len).atan2(self.y / len)
    }

    pub fn normalize(&self) -> Vector {
        let len = self.length();
        Vector::new(self.x / len, self.y / len)
    }

    pub fn between(v1: Vector, v2: Vector) -> Vector {
        Vector::new(v2.x - v1.x, v2.y - v1.y)
    }

    pub fn middle(v1: Vector, v2: Vector) -> Vector {
        Vector::new((v2.x + v1.x) * 0.5, (v2.y + v1.y) * 0.5)
    }

    pub fn rotate_around(&self, center: Vector, angle: f32) -> Vector {
        let mut new_point = self.clone();

        let s = (-angle).sin();
        let c = (-angle).cos();
        // translate point back to origin:
        new_point.x -= center.x;
        new_point.y -= center.y;
        // rotate point
        let new_x = new_point.x * c - new_point.y * s;
        let new_y = new_point.x * s + new_point.y * c;
        // translate point back:
        new_point.x = new_x + center.x;
        new_point.y = new_y + center.y;
        new_point
    }
}

const ANGLE45: f32 = 0.785398;
const HALF_SLOPE: f32 = 0.4636476;
const DOUBLE_SLOPE: f32 = 1.10714872;
use std::f32::consts::PI;
const SQRT2: f32 = 1.41421;
const SQRT5: f32 = 2.236068;

pub fn rad_to_deg(angle: f32) -> f32 {
    angle * (180.0 / PI)
}

pub fn deg_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn create_triangle(verts: [Vector; 3]) -> Vec<GDObject> {
    let mut objects = Vec::<GDObject>::new();

    //find angles
    let mut angles = [0.0; 3];

    for i in 0..3 {
        let v0 = if i == 0 { verts[2] } else { verts[i - 1] };
        let v1 = verts[i];
        let v2 = if i == 2 { verts[0] } else { verts[i + 1] };
        let vec1 = Vector::new(v0.x - v1.x, v0.y - v1.y);
        let vec2 = Vector::new(v2.x - v1.x, v2.y - v1.y);
        let angle = (Vector::dot(vec1, vec2) / (vec1.length() * vec2.length())).acos();
        angles[i] = angle;
    }

    println!(
        "{}, {}, {}",
        rad_to_deg(angles[0]),
        rad_to_deg(angles[1]),
        rad_to_deg(angles[2])
    );

    //45 deg = 0.785398

    //if all angles are over 45 deg: three 45 deg slopes

    let one_small_angle = |angle_index: usize| -> Vec<GDObject> {
        let mut inner_objects = Vec::<GDObject>::new();
        for i in 0..2 {
            let v1 = verts[angle_index];
            let v2 = if angle_index == 0 {
                if angle_index == 2 {
                    verts[0]
                } else {
                    verts[angle_index + 1]
                }
            } else {
                if angle_index == 0 {
                    verts[2]
                } else {
                    verts[angle_index - 1]
                }
            };

            let mut obj = GDObject::new("694");
            let between = Vector::between(v1, v2);
            obj.set_prop(32, &(between.length() / SQRT5).to_string());

            obj.set_pos(Vector::middle(v1, v2));

            let angle = between.angle();
            if i == 0 {
                //no flip
                obj.set_prop(6, &(rad_to_deg(angle + HALF_SLOPE) as i32 - 90).to_string());
            } else {
                //flip
                obj.set_prop(6, &(rad_to_deg(angle - HALF_SLOPE) as i32 - 90).to_string());
                obj.set_prop(5, "1");
            };
            inner_objects.push(obj);
        }

        //last side
        let v2 = if angle_index == 0 {
            verts[2]
        } else {
            verts[angle_index - 1]
        };
        let v1 = if angle_index == 2 {
            verts[0]
        } else {
            verts[angle_index + 1]
        };

        let between = Vector::new(v2.x - v1.x, v2.y - v1.y);

        let pos = Vector::new((v1.x + v2.x) * 0.5, (v1.y + v2.y) * 0.5);
        let angle_deg = rad_to_deg(between.angle()) as i32 - 45;

        let scale = between.length() / SQRT2;

        let mut obj = GDObject::new("693");
        obj.set_pos(pos);
        obj.set_prop(6, &angle_deg.to_string());
        obj.set_prop(32, &scale.to_string());
        inner_objects.push(obj);
        inner_objects
    };

    let one_big_angle = |angle_index: usize| -> Vec<GDObject> {
        let mut inner_objects = Vec::<GDObject>::new();
        //exact same as small angle, except the slopes are flipped
        println!("BIG ANGLE: {}", angle_index);
        for i in 0..2 {
            let v2 = verts[angle_index];
            let v1 = if i == 0 {
                if angle_index == 2 {
                    println!("ANGLE 2: {}", 0);
                    verts[0]
                } else {
                    println!("ANGLE 2: {}", angle_index + 1);
                    verts[angle_index + 1]
                }
            } else {
                if angle_index == 0 {
                    println!("ANGLE 2: {}", 2);
                    verts[2]
                } else {
                    println!("ANGLE 2: {}", angle_index - 1);
                    verts[angle_index - 1]
                }
            };

            let mut obj = GDObject::new("694");
            let between = Vector::between(v1, v2);
            obj.set_prop(32, &(between.length() / SQRT5).to_string());

            obj.set_pos(Vector::middle(v1, v2));

            let angle = between.angle();
            if i == 1 {
                //no flip
                obj.set_prop(6, &(rad_to_deg(angle + HALF_SLOPE) as i32 - 90).to_string());
            } else {
                //flip
                obj.set_prop(6, &(rad_to_deg(angle - HALF_SLOPE) as i32 - 90).to_string());
                obj.set_prop(5, "1");
            };
            inner_objects.push(obj);
        }

        //last side
        let vec2 = if angle_index == 0 {
            verts[2]
        } else {
            verts[angle_index - 1]
        };
        let vec1 = if angle_index == 2 {
            verts[0]
        } else {
            verts[angle_index + 1]
        };
        let middle = Vector::middle(vec1, vec2);

        let angle = Vector::between(vec1, vec2).angle();

        for i in 0..2 {
            let v1 = if i == 0 { vec1 } else { middle };
            let v2 = if i == 0 { middle } else { vec2 };

            let mut obj = GDObject::new("694");
            let between = Vector::between(v1, v2);
            obj.set_prop(32, &(between.length() / 2.0).to_string());

            let center_of_rot = Vector::middle(v1, v2);

            let pos = Vector::new(center_of_rot.x, center_of_rot.y);
            //.rotate_around(center_of_rot, angle + deg_to_rad(90.0));

            obj.set_pos(pos);

            if i == 1 {
                obj.set_prop(6, &(rad_to_deg(angle) as i32 + 90).to_string());
            } else {
                obj.set_prop(6, &(rad_to_deg(angle) as i32 - 90).to_string());
                obj.set_prop(5, "1");
            };
            inner_objects.push(obj);
        }

        inner_objects
    };

    if angles[0] >= ANGLE45 && angles[1] >= ANGLE45 && angles[2] >= ANGLE45 {
        for i in 0..3 {
            let v1 = verts[i];
            let v2 = if i == 2 { verts[0] } else { verts[i + 1] };

            let between = Vector::new(v2.x - v1.x, v2.y - v1.y);

            let pos = Vector::new((v1.x + v2.x) * 0.5, (v1.y + v2.y) * 0.5);
            let angle_deg = rad_to_deg(between.angle()) as i32 - 45;

            let scale = between.length() / SQRT2;

            let mut obj = GDObject::new("693");
            obj.set_pos(pos);
            obj.set_prop(6, &angle_deg.to_string());
            obj.set_prop(32, &scale.to_string());
            objects.push(obj);
        }
    } else if angles[0] >= HALF_SLOPE && angles[1] >= DOUBLE_SLOPE && angles[2] >= DOUBLE_SLOPE {
        objects.extend(one_small_angle(0));
    } else if angles[1] >= HALF_SLOPE && angles[2] >= DOUBLE_SLOPE && angles[0] >= DOUBLE_SLOPE {
        objects.extend(one_small_angle(1));
    } else if angles[2] >= HALF_SLOPE && angles[0] >= DOUBLE_SLOPE && angles[1] >= DOUBLE_SLOPE {
        objects.extend(one_small_angle(2));
    } else if angles[0] >= DOUBLE_SLOPE && angles[1] >= HALF_SLOPE && angles[2] >= HALF_SLOPE {
        objects.extend(one_big_angle(0));
    } else if angles[1] >= DOUBLE_SLOPE && angles[2] >= HALF_SLOPE && angles[0] >= HALF_SLOPE {
        objects.extend(one_big_angle(1));
    } else if angles[2] >= DOUBLE_SLOPE && angles[0] >= HALF_SLOPE && angles[1] >= HALF_SLOPE {
        objects.extend(one_big_angle(2));
    } else {
        //line method
        //find longest edge and its oposite point
        //do the thing
        let mut longest = Vector::new(0.0, 0.0);
        let mut longest_length = 0.0;
        let mut op_point = Vector::new(0.0, 0.0); //oposite point
        let mut longest_orig = Vector::new(0.0, 0.0);

        for i in 0..3 {
            let p1 = verts[i];
            let p2 = if i == 2 { verts[0] } else { verts[i + 1] };
            let between = Vector::new(p2.x - p1.x, p2.y - p1.y);
            let len = between.length();

            if len > longest_length {
                longest_length = len;
                longest = between;
                op_point = if i == 0 { verts[2] } else { verts[i - 1] };
                longest_orig = p1;
            }
        }

        //println!("{}x + {}", line_a, line_b);

        let dist_to_op = point_to_line(longest_orig, longest, op_point);

        println!("{}", dist_to_op);

        let longest_center = Vector::new(
            longest_orig.x + longest.x / 2.0,
            longest_orig.y + longest.y / 2.0,
        );

        let center_to_center =
            Vector::new(op_point.x - longest_center.x, op_point.y - longest_center.y);

        let c_to_c_len = center_to_center.length();

        let ratio = c_to_c_len / dist_to_op;
        //println!("ratio: {}", ratio);

        let mut progress = 1.0; //0.0 = is at op_point

        let angle_deg = rad_to_deg(longest.angle()) - 90.0;

        loop {
            let mut line_obj = GDObject::new("1753");
            let scale = progress * longest_length * 0.95;
            if scale < 0.3 {
                break;
            }

            //half line thickness along the center-to-center line
            let half_line_thickness = ((scale / 60.0) / ratio) / c_to_c_len;

            progress -= half_line_thickness; //progress half the line thckness

            line_obj.set_pos(Vector::new(
                op_point.x - center_to_center.x * progress,
                op_point.y - center_to_center.y * progress,
            ));

            progress -= half_line_thickness;

            line_obj.set_prop(6, &angle_deg.to_string());
            line_obj.set_prop(32, &(scale).to_string());

            objects.push(line_obj);
            //println!("{}", progress);
        }

        //lines on the two other edges

        for point in [
            longest_orig,
            Vector::new(longest_orig.x + longest.x, longest_orig.y + longest.y),
        ]
        .iter()
        {
            let mut line_obj = GDObject::new("1753");
            line_obj.set_pos(Vector::new(
                (op_point.x + point.x) * 0.5,
                (op_point.y + point.y) * 0.5,
            ));

            let between = Vector::new(op_point.x - point.x, op_point.y - point.y);
            line_obj.set_prop(6, &(rad_to_deg(between.angle()) as i32 - 90).to_string());
            line_obj.set_prop(32, &between.length().to_string());
            objects.push(line_obj);
        }
    }

    //else if one angle between 45 and 22.5 or whatever it is,
    //and another angle over the sum of the two: use 22.5 slope

    //else use lines probably
    objects
}

pub fn point_to_line(line_orig: Vector, line: Vector, point: Vector) -> f32 {
    let b = Vector::new(point.x - line_orig.x, point.y - line_orig.y);
    let b_length = b.length();

    let a = Vector::new(-line.y, line.x).normalize();

    let x = Vector::dot(line, b) / (line.length() * b_length);

    //let ratio = (deg_to_rad(90.0) - alpha).cos();

    let ratio = (-(x * x) + 1.0).sqrt();

    let intersect = Vector::new(
        point.x + a.x * b_length * ratio,
        point.y + a.y * b_length * ratio,
    );

    Vector::new(point.x - intersect.x, point.y - intersect.y).length()
}
