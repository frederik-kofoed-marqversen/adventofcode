use std::fs::read_to_string;
use ndarray::{Array1, Array2, Axis};
use ndarray::{s, concatenate, arr1};
use ndarray_linalg::Solve;

type Point3 = [f64; 3];
type DataStruct = Vec<(Point3, Point3)>;

fn main() {
    let hails = parse_file("./input.data").unwrap();
    // dbg!(&hails);

    // PART 1
    let test_area = (200_000_000_000_000.0, 400_000_000_000_000.0);
    let mut num_intersections = 0;

    for (i, (p1, v1)) in hails.iter().enumerate() {
        for (p2, v2) in hails[i+1..].iter() {
            let (t1, t2) = match intersect_paths(p1, p2, v1, v2) {
                None => continue,
                Some(t) => t,
            };

            if t1 < 0.0 || t2 < 0.0 {continue;} // Intersection in the past

            let intersection = [p1[0] + v1[0]*t1, p1[1] + v1[1]*t1];
            // dbg!(&intersection);
            if intersection.iter().all(|&x| x > test_area.0 && x < test_area.1) {
                num_intersections += 1;
            }
        }
    }
    println!("Result part 1: {}", num_intersections);

    // PART 2
    // We seek solution l(t) = p + v*t. 
    // Consider the first hail l0(t) = p0 + v0*t. Intersection l(t0) = l0(t0) for some t0 
    //     implies => p - p0 = t0 * (v0 - v)
    // Taking the cross product with the velocity vector we eliminate the unknown t0 
    //     => (p - p0) x (v0 - v) = 0 => p0 x v0 + p x v = p x v0 + p0 x v
    // This is non-linear only because of the term p x v. This can be eliminated by subtracting the 
    //     equation for the second hail l1 giving => p0 x v0 - p1 x v1 = p x (v0-v1) + (p0-p1) x v
    // These are 3 linear equations with 6 unknowns (p and v). 3 further equations come from 
    //     considering also l0 with l2. The linear system is then solved using standard methods.

    let (p0, v0) = (arr1(&hails[0].0), arr1(&hails[0].1));
    let (p1, v1) = (arr1(&hails[1].0), arr1(&hails[1].1));
    let (p2, v2) = (arr1(&hails[2].0), arr1(&hails[2].1));

    let b1 = cross_product(&p0, &v0) - cross_product(&p1, &v1);
    let b2 = cross_product(&p0, &v0) - cross_product(&p2, &v2);
    let a11 = - cross_product_map(&(&v0-&v1));
    let a12 = cross_product_map(&(&p0-&p1));
    let a21 = - cross_product_map(&(&v0-&v2));
    let a22 = cross_product_map(&(&p0-&p2));

    let b = concatenate![Axis(0), b1, b2];
    let a = concatenate![
        Axis(0), 
        concatenate![Axis(1), a11, a12], 
        concatenate![Axis(1), a21, a22],
    ];

    let result = a.solve_into(b).unwrap();
    let p = result.slice(s![..3]);
    let v = result.slice(s![3..]);
    dbg!(&p, &v);

    // Unfortunately there are no methods for solving the equations over integers, and the result here
    //     is non-integer. The correct solution will be had by rounding. I simply checked both answers.
    // This is unsatisfactory, and should be fixed at some point. (Maybe use Z3 instead of linalg?)
    println!("Result part 2: {}", p.sum());
}

fn cross_product(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
    arr1(&[
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
    ])
}

fn cross_product_map(v: &Array1<f64>) -> Array2<f64> {
    // returns the matrix [v x] which acts like [v x] u = v x u
    Array2::from_shape_vec((3, 3), vec![
        0.0, -v[2], v[1],
        v[2], 0.0, -v[0],
        -v[1], v[0], 0.0,
    ]).unwrap()
}

fn intersect_paths(p1: &[f64], p2: &[f64], v1: &[f64], v2: &[f64]) -> Option<(f64, f64)> {
    // l1 = p1 + t1*v1
    // l2 = p2 + t2*v2

    // l1=l2 leads to matrix equation: a * t = b
    let a = [[-v1[0], v2[0]], [-v1[1], v2[1]]];
    let b = [p1[0]-p2[0], p1[1]-p2[1]];

    let det = a[0][0]*a[1][1] - a[1][0]*a[0][1];
    if det == 0.0 {return None;} // Parallel paths

    // Cramers rule to solve equation for t
    // t1 = det(b | a[..][1]) / det(a)
    // t2 = det(a[..][0], b) / det(a)
    let det1 = b[0]*a[1][1] - b[1]*a[0][1];
    let det2 = a[0][0]*b[1] - a[1][0]*b[0];
    let (t1, t2) = (det1/det, det2/det);
    
    return Some((t1, t2))
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    Ok(read_to_string(filepath)?.trim().replace(' ', "").split('\n').map(
        |line| -> (Point3, Point3) {
            let arrays = line.split('@').map(
                |numbers| numbers.split(',').map(
                    |num| num.parse::<f64>().unwrap()
                ).collect::<Vec<f64>>().try_into().unwrap()
            ).collect::<Vec<Point3>>();
            (arrays[0].clone(), arrays[1].clone())
        }
    ).collect())
}