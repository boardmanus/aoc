use minilp::{ComparisonOp, OptimizationDirection, Problem};
use std::{
    collections::HashMap,
    num::ParseFloatError,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug)]
enum ParseErr {
    NotEnoughElements,
    F64(ParseFloatError),
}

impl From<ParseFloatError> for ParseErr {
    fn from(e: ParseFloatError) -> Self {
        ParseErr::F64(e)
    }
}

fn tuple_from_str(s: &str) -> Result<(f64, f64, f64), ParseErr> {
    s.split(", ")
        .map(|s| s.trim().parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
        .map(|v| {
            Ok((
                *v.first().ok_or(ParseErr::NotEnoughElements)?,
                *v.get(1).ok_or(ParseErr::NotEnoughElements)?,
                *v.get(2).ok_or(ParseErr::NotEnoughElements)?,
            ))
        })?
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

impl Pos {
    const MAX: Pos = Pos {
        x: f64::MAX,
        y: f64::MAX,
        z: f64::MAX,
    };

    const MIN: Pos = Pos {
        x: f64::MIN,
        y: f64::MIN,
        z: f64::MIN,
    };

    fn new(x: f64, y: f64, z: f64) -> Self {
        Pos { x, y, z }
    }

    fn min(&self, other: &Pos) -> Pos {
        Pos::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max(&self, other: &Pos) -> Pos {
        Pos::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    fn from_tuple(t: (f64, f64, f64)) -> Self {
        Pos::new(t.0, t.1, t.2)
    }
}

impl Index<usize> for Pos {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index: {}", idx),
        }
    }
}

impl IndexMut<usize> for Pos {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index: {}", idx),
        }
    }
}

impl Into<[f64; 3]> for Pos {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl FromStr for Pos {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pos::from_tuple(tuple_from_str(s)?))
    }
}
#[derive(Debug, PartialEq)]

struct Vel {
    x: f64,
    y: f64,
    z: f64,
}

impl Vel {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vel { x, y, z }
    }

    fn from_tuple(t: (f64, f64, f64)) -> Self {
        Vel::new(t.0, t.1, t.2)
    }
}

impl Index<usize> for Vel {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index: {}", idx),
        }
    }
}

impl Into<[f64; 3]> for Vel {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl IndexMut<usize> for Vel {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index: {}", idx),
        }
    }
}

impl FromStr for Vel {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Vel::from_tuple(tuple_from_str(s)?))
    }
}

fn sub(u: &[f64], v: &[f64]) -> Vec<f64> {
    vec![u[0] - v[0], u[1] - v[1], u[2] - v[2]]
}

fn exterior3(u: &[f64], v: &[f64], w: &[f64]) -> f64 {
    u[0] * v[1] * w[2] + u[1] * v[2] * w[0] + u[2] * v[0] * w[1]
        - u[0] * v[2] * w[1]
        - u[1] * v[0] * w[2]
        - u[2] * v[1] * w[0]
}

fn exterior2(v: &[f64], w: &[f64]) -> Vec<f64> {
    vec![
        v[0] * w[1] - v[1] * w[0],
        v[1] * w[2] - v[2] * w[1],
        v[2] * w[0] - v[0] * w[2],
    ]
}

fn calc_start(hailstones: &[Hail]) -> Pos {
    let d: Vec<Vec<Vec<f64>>> = vec![
        vec![
            vec![
                hailstones[0].pos.x as f64,
                hailstones[0].pos.y as f64,
                hailstones[0].pos.z as f64,
            ],
            vec![
                hailstones[0].vel.x as f64,
                hailstones[0].vel.y as f64,
                hailstones[0].vel.z as f64,
            ],
        ],
        vec![
            vec![
                hailstones[1].pos.x as f64,
                hailstones[1].pos.y as f64,
                hailstones[1].pos.z as f64,
            ],
            vec![
                hailstones[1].vel.x as f64,
                hailstones[1].vel.y as f64,
                hailstones[1].vel.z as f64,
            ],
        ],
        vec![
            vec![
                hailstones[2].pos.x as f64,
                hailstones[2].pos.y as f64,
                hailstones[2].pos.z as f64,
            ],
            vec![
                hailstones[2].vel.x as f64,
                hailstones[2].vel.y as f64,
                hailstones[2].vel.z as f64,
            ],
        ],
    ];
    let a: Vec<Vec<f64>> = vec![
        exterior2(&sub(&d[0][1], &d[1][1]), &sub(&d[0][0], &d[1][0])),
        exterior2(&sub(&d[0][1], &d[2][1]), &sub(&d[0][0], &d[2][0])),
        exterior2(&sub(&d[1][1], &d[2][1]), &sub(&d[1][0], &d[2][0])),
    ];
    let b: Vec<f64> = vec![
        -exterior3(&d[0][0], &d[0][1], &d[1][0]) - exterior3(&d[1][0], &d[1][1], &d[0][0]),
        -exterior3(&d[0][0], &d[0][1], &d[2][0]) - exterior3(&d[2][0], &d[2][1], &d[0][0]),
        -exterior3(&d[1][0], &d[1][1], &d[2][0]) - exterior3(&d[2][0], &d[2][1], &d[1][0]),
    ];
    let det_a =
        a[0][0] * a[1][1] * a[2][2] - a[0][0] * a[1][2] * a[2][1] - a[0][1] * a[1][0] * a[2][2]
            + a[0][1] * a[1][2] * a[2][0]
            + a[0][2] * a[1][0] * a[2][1]
            - a[0][2] * a[1][1] * a[2][0];
    let det_ax = b[0] * a[1][1] * a[2][2] - b[0] * a[1][2] * a[2][1] - a[0][1] * b[1] * a[2][2]
        + a[0][1] * a[1][2] * b[2]
        + a[0][2] * b[1] * a[2][1]
        - a[0][2] * a[1][1] * b[2];
    let det_ay = a[0][0] * b[1] * a[2][2] - a[0][0] * a[1][2] * b[2] - b[0] * a[1][0] * a[2][2]
        + b[0] * a[1][2] * a[2][0]
        + a[0][2] * a[1][0] * b[2]
        - a[0][2] * b[1] * a[2][0];
    let det_az = a[0][0] * a[1][1] * b[2] - a[0][0] * b[1] * a[2][1] - a[0][1] * a[1][0] * b[2]
        + a[0][1] * b[1] * a[2][0]
        + b[0] * a[1][0] * a[2][1]
        - b[0] * a[1][1] * a[2][0];
    let x = det_ax / det_a;
    let y = det_ay / det_a;
    let z = det_az / det_a;
    Pos::new(x, y, z)
}

#[derive(Debug, PartialEq)]
struct Hail {
    pos: Pos,
    vel: Vel,
}

impl Hail {
    fn new(pos: Pos, vel: Vel) -> Self {
        Hail { pos, vel }
    }

    fn path_intersect2d(&self, other: &Hail) -> Option<Pos> {
        let m0 = self.vel.y / self.vel.x;
        let m1 = other.vel.y / other.vel.x;
        let c0 = self.pos.y - m0 * self.pos.x;
        let c1 = other.pos.y - m1 * other.pos.x;
        let denom = m0 - m1;
        if denom.abs() < f64::EPSILON {
            return None;
        }

        let x = (c1 - c0) / denom;
        let y = m0 * x + c0;

        let n0 = ((x - self.pos.x) / self.vel.x, (y - self.pos.y) / self.vel.y);

        let n1 = (
            (x - other.pos.x) / other.vel.x,
            (y - other.pos.y) / other.vel.y,
        );

        if n0.0 < 0.0 || n0.1 < 0.0 || n1.0 < 0.0 || n1.1 < 0.0 {
            return None;
        }

        Some(Pos::new(x, y, 0.0))
    }
}

impl FromStr for Hail {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" @ ");
        let pos = split
            .next()
            .ok_or(ParseErr::NotEnoughElements)?
            .parse::<Pos>()?;
        let vel = split
            .next()
            .ok_or(ParseErr::NotEnoughElements)?
            .parse::<Vel>()?;
        Ok(Hail::new(pos, vel))
    }
}

#[derive(Debug)]
struct Area {
    min: Pos,
    max: Pos,
}

impl Area {
    fn new(min: Pos, max: Pos) -> Self {
        Area { min, max }
    }

    fn contains(&self, pos: &Pos) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
    }

    fn include(&self, pos: &Pos) -> Area {
        Area::new(self.min.min(&pos), self.max.max(&pos))
    }
}

#[derive(Debug)]
struct Storm {
    hail: Vec<Hail>,
}

fn update_pos_neg(hail: &Hail, i: usize, pos: &mut Area, neg: &mut Area) {
    if hail.vel[i] > 0.0 {
        pos.min[i] = pos.min[i].min(hail.pos[i]);
        pos.max[i] = pos.max[i].max(hail.pos[i]);
    } else {
        neg.min[i] = neg.min[i].min(hail.pos[i]);
        neg.max[i] = neg.max[i].max(hail.pos[i]);
    }
}

type CoordType = num::rational::Ratio<i128>;

impl Storm {
    fn bounds(&self) -> Area {
        self.hail
            .iter()
            .fold(Area::new(Pos::MAX, Pos::MIN), |area, hail| {
                area.include(&hail.pos)
            })
    }

    fn solve_hail_paths_cross_product(&self) -> Hail {
        // Start with equestion of a line for the magic hail bullet:
        // P = Ps + t * Vs
        // where P is an arbitray point on the line, s denotes the solution, and t is
        // the nanoseconds of time elapsed. Here, t = [0..].
        // For a known hailstone, we get:
        // P = P0 + t * V0
        // Where 0 denotes the first hailstone in the list.
        // We are looking for the intersection of the lines for a common t.
        // => Ps + t * Vs = P0 + t * V0
        // => 0 = (Ps - P0) + t * (Vs - V0)
        // Apply cross-product (Vs - V0) to both sides, which will cancel the t term
        // => 0 = (Ps - P0) x (Vs - V0)
        // => 0 = Ps x Vs - Ps x V0 - P0 x Vs + P0 x V0
        // => Ps x Vs = Ps x V0 + P0 x Vs - P0 x V0
        // Where Ps x Vs is constant for all hail stones, so do the same for H1
        // => Ps x V0 + P0 x Vs - P0 x V0 = Ps x V1 + P1 x Vs - P1 x V1
        // => Ps x (V0 - V1) + (P0 - P1) x Vs - P0 x V0 + P1 x V1 = 0
        // Apply cross-product (P0 - P1) to elimiate Vs
        // => Ps x (V0 - V1) x (P0 - P1) - P0 x V0 x (P0 - P1) + P1 x V1 x (P0 - P1) = 0
        Hail::new(Pos::new(0.0, 0.0, 0.0), Vel::new(0.0, 0.0, 0.0))
    }
    fn solve_hail_paths_linear(&self) -> Hail {
        // Start with equestion of a line for the magic hail bullet:
        // P = Ps + t * Vs
        // where P is an arbitray point on the line, s denotes the solution, and t is
        // the nanoseconds of time elapsed. Here, t = [0..].
        // For a known hailstone, we get:
        // P = P0 + t * V0
        // Where 0 denotes the first hailstone in the list.
        // We are looking for the intersection of the lines for a common t.
        // Ps + t * Vs = P0 + t * V0
        // => Ps = P0 + t * (V0 - Vs)
        // 1. xs = x0  + t * (vx0 - vxs) => t = (xs - x0) / (vx0 - vxs)
        // 2. ys = y0  + t * (vy0 - vys) => t = (ys - y0) / (vy0 - vys)
        // =>                   (xs - x0) / (vx0 - vxs) = (ys - y0) / (vy0 - vys)
        // =>                   (xs - x0) * (vy0 - vys) = (ys - y0) * (vx0 - vxs)
        // => xs * vy0 - xs * vys - x0 * vy0 + x0 * vys = ys * vx0 - ys * vxs - y0 * vx0 + y0 * vxs
        // =>            xs*vys - ys*vxs =  xs*vy0 - x0*vy0 + x0*vys - ys*vx0 + y0*vx0 -y0*vxs
        // Where xs*vys - ys*vxs is constant for any hailstone it's compared to.
        // So for two hailstones, H0, and H1, we get:
        // => xs*vy0 - x0*vy0 + x0*vys - ys*vx0 + y0*vx0 -y0*vxs = xs*vy1 - x1*vy1 + x1*vys - ys*vx1 + y1*vx1 - y1*vxs
        // => xs*(vy1 - vy0) + vxs*(y0 - y1) + ys*(vx0 - vx1) + vys*(x1 - x0) - x1*vy1 + x0*vy0 - y0*vx0 + y1*vx1 = 0
        // Where the right hand side is constant, and left has 4 unknowns (xs, ys, vxs, vys).
        let terms = |h0: &Hail, h1: &Hail| {
            [
                h1.vel.y - h0.vel.y,
                h0.pos.y - h1.pos.y,
                h0.vel.x - h1.vel.x,
                h1.pos.x - h0.pos.x,
                -h1.pos.x * h1.vel.y + h0.pos.x * h0.vel.y - h0.pos.y * h0.vel.x
                    + h1.pos.y * h1.vel.x,
            ]
        };
        let root_solns = [
            terms(&self.hail[0], &self.hail[1]),
            terms(&self.hail[1], &self.hail[2]),
            terms(&self.hail[2], &self.hail[3]),
            terms(&self.hail[3], &self.hail[4]),
        ];

        let x_terms = |x0: &[f64], x1: &[f64]| {
            [
                x0[0] * x1[1] - x1[0] * x0[1],
                x0[0] * x1[2] - x1[0] * x0[2],
                x0[0] * x1[3] - x1[0] * x0[3],
                x0[0] * x1[4] - x1[0] * x0[4],
            ]
        };
        let x_solns = [
            x_terms(&root_solns[0], &root_solns[1]),
            x_terms(&root_solns[1], &root_solns[2]),
            x_terms(&root_solns[2], &root_solns[3]),
        ];

        let vx_terms = |vx0: &[f64], vx1: &[f64]| {
            [
                vx0[0] * vx1[1] - vx1[0] * vx0[1],
                vx0[0] * vx1[2] - vx1[0] * vx0[2],
                vx0[0] * vx1[3] - vx1[0] * vx0[3],
            ]
        };
        let vx_solns = [
            vx_terms(&x_solns[0], &x_solns[1]),
            vx_terms(&x_solns[1], &x_solns[2]),
        ];

        let y_terms =
            |y0: &[f64], y1: &[f64]| [y0[0] * y1[1] - y1[0] * y0[1], y0[0] * y1[2] - y1[0] * y0[2]];

        let y_solns = [y_terms(&vx_solns[0], &vx_solns[1])];

        let vys = -y_solns[0][1] / y_solns[0][0];
        let ys = -(vx_solns[0][1] * vys + vx_solns[0][2]) / vx_solns[0][0];
        let vxs = -(x_solns[0][1] * ys + x_solns[0][2] * vys + x_solns[0][0]) / x_solns[0][0];
        let xs = -(root_solns[0][1] * vxs + root_solns[0][2] * ys + root_solns[0][3] * vys)
            / root_solns[0][0];
        let t = (xs - self.hail[0].pos.x) / (self.hail[0].vel.x - vxs);

        // zs = z0  + t * (vz0 - vzs) <=> vzs = (z0 - zs)/t + vz0
        // => z0 + t*(vz0 - vzs) = z1 + t*(vz1 - vzs)
        // => (z0 - zs)/t + vz0 = (z1 - zs)/t + vz0
        // => z0 - zs + t*vz0 = z1 - zs + t*vz1

        Hail::new(Pos::new(xs, ys, 0.0), Vel::new(vxs, vys, 0.0))
    }

    fn solve_hail_paths_with_minilp(&self) -> Hail {
        // Start with equestion of a line for the magic hail bullet:
        // P = Ps + t * Vs
        // where P is an arbitray point on the line, s denotes the solution, and t is
        // the nanoseconds of time elapsed. Here, t = [0..].
        // For a known hailstone, we get:
        // P = P0 + t * V0
        // Where 0 denotes the first hailstone in the list.
        // We are looking for the intersection of the lines for a common t.
        // Ps + t * Vs = P0 + t * V0
        // => Ps = P0 + t * (V0 - Vs)
        // 1. xs = x0  + t * (vx0 - vxs) => t = (xs - x0) / (vx0 - vxs)
        // 2. ys = y0  + t * (vy0 - vys) => t = (ys - y0) / (vy0 - vys)
        // =>         (xs - x0) / (vx0 - vxs) = (ys - y0) / (vy0 - vys)
        // =>         (xs - x0) * (vy0 - vys) = (ys - y0) * (vx0 - vxs)
        // => xs * vy0 - xs * vys - x0 * vy0 + x0 * vys = ys * vx0 - ys * vxs - y0 * vx0 + y0 * vxs
        // =>                           xs*vys - ys*vxs = xs*vy0 - x0*vy0 + x0*vys - ys*vx0 + y0*vx0 -y0*vxs
        // Where xs*vys - ys*vxs is constant for any hailstone it's compared to.
        // So for two hailstones, H0, and H1, we get:
        // => xs*vy0 - x0*vy0 + x0*vys - ys*vx0 + y0*vx0 -y0*vxs = xs*vy1 - x1*vy1 + x1*vys - ys*vx1 + y1*vx1 - y1*vxs
        // => xs*(vy1 - vy0) + vxs*(y0 - y1) + ys*(vx0 - vx1) + vys*(x1 - x0) - x1*vy1 + x0*vy0 - y0*vx0 + y1*vx1 = 0
        // and to get the z variables:
        // => ys*(vz1 - vz0) + vys*(z0 - z1) + zs*(vy0 - vy1) + vzs*(y1 - y0) - y1*vz1 + y0*vz0 - z0*vy0 + z1*vy1 = 0
        let mut problem = Problem::new(OptimizationDirection::Maximize);
        let xs = problem.add_var(1.0, (0.0, f64::INFINITY));
        let ys = problem.add_var(1.0, (0.0, f64::INFINITY));
        let zs = problem.add_var(1.0, (0.0, f64::INFINITY));
        let vxs = problem.add_var(1.0, (0.0, f64::INFINITY));
        let vys = problem.add_var(1.0, (0.0, f64::INFINITY));
        let vzs = problem.add_var(1.0, (0.0, f64::INFINITY));

        for i in (1..) {
            let h0 = &self.hail[i - 1];
            let h1 = &self.hail[i];
            let xy_constraint = &[
                (xs, h1.vel.y - h0.vel.y),
                (vxs, h0.pos.y - h1.pos.y),
                (ys, h0.vel.x - h1.vel.x),
                (vys, h1.pos.x - h0.pos.x),
            ];
            // x1*vy1 - x0*vy0 + y0*vx0 - y1*vx1;
            let xy_res = h1.pos.x * h1.vel.y - h0.pos.x * h0.vel.y + h0.pos.y * h0.vel.x
                - h1.pos.y * h1.vel.x;

            let yz_constraint = &[
                (ys, h1.vel.z - h0.vel.z),
                (vys, h0.pos.z - h1.pos.z),
                (zs, h0.vel.y - h1.vel.y),
                (vzs, h1.pos.y - h0.pos.y),
            ];
            let yz_res = h1.pos.y * h1.vel.z - h0.pos.y * h0.vel.z + h0.pos.z * h0.vel.y
                - h1.pos.z * h1.vel.y;

            problem.add_constraint(xy_constraint, ComparisonOp::Eq, xy_res);
            problem.add_constraint(yz_constraint, ComparisonOp::Eq, yz_res);

            if let Ok(soln) = problem.solve() {
                return Hail::new(
                    Pos::new(soln[xs], soln[ys], soln[zs]),
                    Vel::new(soln[vxs], soln[vys], soln[vzs]),
                );
            }
        }

        Hail::new(Pos::new(0.0, 0.0, 0.0), Vel::new(0.0, 0.0, 0.0))
    }

    fn solve_hail_paths_gauss(&self) -> CoordType {
        // f64 isn't precise enough, as well as leading to trouble when comparing to 0. Rational64 isn't big enough.
        let map_hail = |hail: &Hail| {
            (
                CoordType::from_integer(hail.pos.x as i128),
                CoordType::from_integer(hail.pos.y as i128),
                CoordType::from_integer(hail.pos.z as i128),
                CoordType::from_integer(hail.vel.x as i128),
                CoordType::from_integer(hail.vel.y as i128),
                CoordType::from_integer(hail.vel.z as i128),
            )
        };

        let (p_ax, p_ay, p_az, v_ax, v_ay, v_az) = map_hail(&self.hail[0]);
        let (p_bx, p_by, p_bz, v_bx, v_by, v_bz) = map_hail(&self.hail[1]);
        let (p_cx, p_cy, p_cz, v_cx, v_cy, v_cz) = map_hail(&self.hail[2]);

        let mut equations = [
            [
                CoordType::default(),
                v_az - v_cz,
                v_cy - v_ay,
                CoordType::default(),
                p_cz - p_az,
                p_ay - p_cy,
                p_ay * v_az - p_az * v_ay - p_cy * v_cz + p_cz * v_cy,
            ],
            [
                v_az - v_cz,
                CoordType::default(),
                v_cx - v_ax,
                p_cz - p_az,
                CoordType::default(),
                p_ax - p_cx,
                p_ax * v_az - p_az * v_ax - p_cx * v_cz + p_cz * v_cx,
            ],
            [
                v_cy - v_ay,
                v_ax - v_cx,
                CoordType::default(),
                p_ay - p_cy,
                p_cx - p_ax,
                CoordType::default(),
                p_ay * v_ax - p_ax * v_ay - p_cy * v_cx + p_cx * v_cy,
            ],
            [
                CoordType::default(),
                v_bz - v_cz,
                v_cy - v_by,
                CoordType::default(),
                p_cz - p_bz,
                p_by - p_cy,
                p_by * v_bz - p_bz * v_by - p_cy * v_cz + p_cz * v_cy,
            ],
            [
                v_bz - v_cz,
                CoordType::default(),
                v_cx - v_bx,
                p_cz - p_bz,
                CoordType::default(),
                p_bx - p_cx,
                p_bx * v_bz - p_bz * v_bx - p_cx * v_cz + p_cz * v_cx,
            ],
            [
                v_cy - v_by,
                v_bx - v_cx,
                CoordType::default(),
                p_by - p_cy,
                p_cx - p_bx,
                CoordType::default(),
                p_by * v_bx - p_bx * v_by - p_cy * v_cx + p_cx * v_cy,
            ],
        ];

        // Perform gaussian elimination
        // Iterate diagonally from top left, to turn matrix into reduced row echelon form
        for i in 0..6 {
            // Find non-zero item in current column, from current row or after
            let non_zero_row = (i..6)
                .find(|&row| equations[row][i] != CoordType::default())
                .unwrap();

            // Swap current row with first non-zero row
            if non_zero_row != i {
                (equations[i], equations[non_zero_row]) = (equations[non_zero_row], equations[i]);
            }

            // Divide row by value at current pos, to turn value into 1
            let curr_val = equations[i][i];
            equations[i][i] = CoordType::from_integer(1);
            for item in &mut equations[i][i + 1..] {
                *item /= curr_val;
            }

            // Subtract multiple of current row from lower rows, to turn column below current item to 0
            for row in i + 1..6 {
                let multiple = equations[row][i];
                equations[row][i] = CoordType::default();
                if multiple != CoordType::default() {
                    for col in i + 1..7 {
                        equations[row][col] -= equations[i][col] * multiple;
                    }
                }
            }
        }

        // Iterate diagonally from bottom right, to turn matrix (except last column) into unit matrix.
        for i in (0..6).rev() {
            for row in 0..i {
                equations[row][6] -= equations[i][6] * equations[row][i];
                equations[row][i] = CoordType::default();
            }
        }

        equations.iter().take(3).map(|x| x[6]).sum::<CoordType>()
    }
}

impl FromStr for Storm {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Storm {
            hail: s
                .lines()
                .map(|l| l.parse::<Hail>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn count_inbound_intersections(storm: &Storm, bounds: &Area) -> usize {
    storm.hail[0..storm.hail.len() - 1]
        .iter()
        .enumerate()
        .fold(0, |intersections, (i, hail)| {
            let intersecting_paths = storm.hail[i + 1..]
                .iter()
                .filter(|hail2| {
                    hail.path_intersect2d(hail2)
                        .map(|p| bounds.contains(&p))
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();

            intersections + intersecting_paths.len()
        })
}

fn solve_part1(input: &str) -> usize {
    count_inbound_intersections(
        &Storm::from_str(input).unwrap(),
        &Area::new(
            Pos::new(200000000000000.0, 200000000000000.0, 0.0),
            Pos::new(400000000000000.0, 400000000000000.0, 0.0),
        ),
    )
}

fn solve_part2_fail(input: &str) -> usize {
    let storm = Storm::from_str(input).unwrap();
    //let hail = storm.solve_hail_paths();
    //println!("Hail: {:?}", hail);
    //(hail.pos.x + hail.pos.y + hail.pos.z) as usize
    let mut xs: HashMap<usize, usize> = HashMap::new();
    let mut ys: HashMap<usize, usize> = HashMap::new();
    let mut zs: HashMap<usize, usize> = HashMap::new();
    (0..(storm.hail.len() - 3)).for_each(|i| {
        let raw_pos = calc_start(&storm.hail[i..]);
        let pos = (
            raw_pos.x.round() as usize,
            raw_pos.y.round() as usize,
            raw_pos.z.round() as usize,
        );

        xs.entry(pos.0).and_modify(|e| *e += 1).or_insert(1);
        ys.entry(pos.1).and_modify(|e| *e += 1).or_insert(1);
        zs.entry(pos.2).and_modify(|e| *e += 1).or_insert(1);
    });
    let x = xs.iter().max().unwrap().0;
    let y = ys.iter().max().unwrap().0;
    let z = zs.iter().max().unwrap().0;
    println!(
        "xs={:?}",
        xs.iter()
            .filter(|(_k, &v)| v > 30)
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<_>>()
    );
    println!(
        "ys={:?}",
        ys.iter()
            .filter(|(_k, &v)| v > 30)
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<_>>()
    );
    println!(
        "zs={:?}",
        zs.iter()
            .filter(|(_k, &v)| v > 30)
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<_>>()
    );
    x + y + z
}

fn solve_part2(input: &str) -> CoordType {
    let storm = Storm::from_str(input).unwrap();
    storm.solve_hail_paths_gauss()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), CoordType::from_integer(47));
    }

    #[test]
    fn test_parse_pos() {
        let pos = Pos::from_str("1, 2, 3").unwrap();
        assert_eq!(pos, Pos::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_parse() {
        let storm = Storm::from_str(TEST_INPUT).unwrap();
        assert_eq!(storm.hail.len(), 5);
        assert_eq!(
            storm.hail[2],
            Hail::new(Pos::new(20.0, 25.0, 34.0), Vel::new(-2.0, -2.0, -4.0))
        );
    }

    fn approx_eq_2d(p0: &Pos, p1: &Pos) -> bool {
        format!("{:.3}", p0.x) == format!("{:.3}", p1.x)
            && format!("{:.3}", p0.y) == format!("{:.3}", p1.y)
    }

    #[test]
    fn test_path_intersect() {
        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 18, 19, 22 @ -1, -1, -2
        // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
        let hail0 = Hail::from_str("19, 13, 30 @ -2, 1, -2").unwrap();
        let hail1 = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let p = hail0.path_intersect2d(&hail1).unwrap();
        assert!(approx_eq_2d(&p, &Pos::new(14.333, 15.333, 0.0)));

        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        // Hailstones' paths crossed in the past for hailstone A.
        let hail0 = Hail::from_str("19, 13, 30 @ -2, 1, -2").unwrap();
        let hail1 = Hail::from_str("20, 19, 15 @ 1, -5, -3").unwrap();
        assert_eq!(hail0.path_intersect2d(&hail1), None);

        // Hailstone A: 18, 19, 22 @ -1, -1, -2
        // Hailstone B: 12, 31, 28 @ -1, -2, -1
        // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
        let hail0 = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let hail1 = Hail::from_str("12, 31, 28 @ -1, -2, -1").unwrap();
        assert_eq!(
            hail0.path_intersect2d(&hail1),
            Some(Pos::new(-6.0, -5.0, 0.0))
        );

        let hail0 = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let hail1 = Hail::from_str("20, 25, 34 @ -2, -2, -4").unwrap();
        assert_eq!(hail0.path_intersect2d(&hail1), None);
    }

    #[test]
    fn test_area_contains() {
        let area = Area::new(Pos::new(7.0, 7.0, 0.0), Pos::new(27.0, 27.0, 0.0));
        assert!(area.contains(&Pos::new(14.333, 15.333, 0.0)));
        assert!(area.contains(&Pos::new(7.0, 7.0, 0.0)));
        assert!(area.contains(&Pos::new(27.0, 27.0, 0.0)));
        assert!(!area.contains(&Pos::new(6.999, 16.0, 0.0)));
        assert!(!area.contains(&Pos::new(16.0, 27.001, 0.0)));
    }

    #[test]
    fn test_storm_bounds() {
        let storm = Storm::from_str(TEST_INPUT).unwrap();
        let area = storm.bounds();
        println!("bounds: {:?}", area);
    }
}
