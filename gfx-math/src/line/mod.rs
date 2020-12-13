use crate::{
    point::{Pt2, Pt3},
    vector::{Vec2, Vec3},
};

#[macro_use]
mod macros;

define_line!(Line2, Pt2, Vec2);
define_line!(Line3, Pt3, Vec3);
