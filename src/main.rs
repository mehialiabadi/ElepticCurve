
mod eleptic_curve;
mod algebra;
use crate::eleptic_curve::EC::{ElepticCurve,Point};

fn main() {



// Define an example curve E: y^2 = x^3 - x  (mod 71)  where the order of subgroup is 72, 71 points on the curve + point at infinity . 
let a:i32=-1;
let b:i32=0;
let p:i32=71; 

let simple_ec=ElepticCurve{a,b,p};

let p1=Point{pointx:5,pointy:1};
let mut my_list:Vec<Point>= Vec::new();
let mut mylist_allpoints:Vec<Point>= Vec::new();

mylist_allpoints=simple_ec.generate_allPointsNaive();
}


