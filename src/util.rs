

#[macro_export]
macro_rules! rotate {
    ($x: expr, $y: expr, $t: expr, $p: expr) => {
        {
            let st = $t.sin();
            let ct = $t.cos();
            Vec2{
                x: (ct * $x - st * $y) + $p.0, 
                y: (st * $x + ct * $y) + $p.1
            }
        }
    };
}

#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, Copy, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

#[macro_export]
macro_rules! hex {
    ($x: expr) => {
        Color::from_hex($x)
    };
}

pub_struct!(PidConstants {
    p: f32,
    i: f32,
    d: f32,
    tolerance: f32,
    integralThreshold: f32,
    maxIntegral: f32,
});

pub struct Pid {
    prevError: f32,
    derivative: f32,
    integral: f32,
    constants: PidConstants
}

impl Pid {
    pub fn new(con: PidConstants) -> Pid {
        Pid {
            prevError:0.0,
            derivative:0.0,
            integral:0.0,
            constants: con
        }
    }

    pub fn out(&mut self, error: f32) -> f32 {
        if error.abs() < self.constants.tolerance {self.integral= 0.0}
        else if error.abs() < self.constants.integralThreshold {self.integral+= error};
        if self.integral > self.constants.maxIntegral {self.integral= self.constants.maxIntegral};
        self.derivative = error - self.prevError;
        self.prevError = error;
        error * self.constants.p  + self.integral* self.constants.i + self.derivative * self.constants.d
    }
}

impl PidConstants {
    pub fn new() -> PidConstants {
        PidConstants {
            p:0.0,
            i:0.0,
            d:0.0,
            tolerance:0.0,
            integralThreshold:0.0,
            maxIntegral: 0.0
        }
    }
}

pub fn dist(a: (f32, f32), b: (f32, f32)) -> f32{
    (a.0 - b.0).hypot(b.1 - a.1)
}

pub fn absoluteAngleToPoint(p1: (f32, f32), p2: (f32, f32)) -> f32{
    use std::f32::consts::PI;
    let c = (p2.1 - p1.1).atan2(p2.0 - p1.0) * 180.0/PI + 90.0;
    if c < 0.0 {360.0 + c} else {c}
}

pub fn dirToSpin(target: f32, current: f32) -> i16{
    let d = target - current;
    let diff = if d < 0.0 {d + 360.0} else {d};
    if diff > 180.0 {1} else {-1}
}

pub fn minError(target: f32, current: f32) -> f32 {
    let b = target.max(current);
    let s = target.min(current);
    let diff = b - s;
    let dir = dirToSpin(target, current) as f32;
    if diff <= 180.0 {diff * dir} else {((360.0-b) + s) * dir}
}

use macroquad::prelude::*;
pub fn draw_arc_lines(center: Vec2, r: f32, start: f32, end: f32, thickness: f32, color: Color) {
    let incr = (end - start) / 20.;
    for i in 0..20 {
        let a = center + polar_to_cartesian(r, start + i as f32 * incr);
        let b = center + polar_to_cartesian(r, start + (i + 1) as f32 * incr);
        draw_line(a.x, a.y, b.x, b.y, thickness, color);
    }
}

pub fn draw_arc(center: Vec2, r: f32, start: f32, end: f32, color: Color) {
    // let center = vec2(center.0, center.1);
    let incr = (end - start) / 20.;
    for i in 0..20 {
        let a = center + polar_to_cartesian(r, start + i as f32 * incr);
        let b = center + polar_to_cartesian(r, start + (i + 1) as f32 * incr);
        draw_triangle(center, a, b, color);
    }
}

pub(crate) use rotate;
