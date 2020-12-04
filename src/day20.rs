use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/20.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl std::fmt::Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p={}, v={}, a={}", self.pos, self.vel, self.acc)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Dim {
    pos: i64,
    vel: i64,
    acc: i64,
}

impl std::fmt::Display for Dim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p={}, v={}, a={}", self.pos, self.vel, self.acc)
    }
}

impl Dim {
    #[cfg(test)]
    fn advance(mut self) -> Dim {
        self.vel += self.acc;
        self.pos += self.vel;
        self
    }
}

struct DimRefMut<'a> {
    pos: &'a mut i64,
    vel: &'a mut i64,
    acc: &'a mut i64,
}

impl<'a> DimRefMut<'a> {
    fn normalize(&mut self) {
        if self.is_neg() {
            *self.pos = -*self.pos;
            *self.vel = -*self.vel;
            *self.acc = -*self.acc;
        }
    }

    fn is_neg(&self) -> bool {
        if *self.acc < 0 {
            true
        } else if *self.acc == 0 {
            if *self.vel < 0 {
                true
            } else if *self.vel == 0 {
                *self.pos < 0
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Particle {
    /// Transforms the particle into a particle which is, for all time, the same distance from
    /// the origin, but also tends to nonnegativity forever
    fn normalize(mut self) -> Particle {
        self.x_mut().normalize();
        self.y_mut().normalize();
        self.z_mut().normalize();
        self
    }

    fn advance(mut self) -> Particle {
        self.vel += self.acc;
        self.pos += self.vel;
        self
    }

    fn x(self) -> Dim {
        Dim {
            pos: self.pos.x,
            vel: self.vel.x,
            acc: self.acc.x,
        }
    }

    fn y(self) -> Dim {
        Dim {
            pos: self.pos.y,
            vel: self.vel.y,
            acc: self.acc.y,
        }
    }

    fn z(self) -> Dim {
        Dim {
            pos: self.pos.z,
            vel: self.vel.z,
            acc: self.acc.z,
        }
    }

    /// Mutable reference to the x-dimension of this particle
    fn x_mut(&mut self) -> DimRefMut<'_> {
        DimRefMut {
            pos: &mut self.pos.x,
            vel: &mut self.vel.x,
            acc: &mut self.acc.x,
        }
    }

    /// Mutable reference to the x-dimension of this particle
    fn y_mut(&mut self) -> DimRefMut<'_> {
        DimRefMut {
            pos: &mut self.pos.y,
            vel: &mut self.vel.y,
            acc: &mut self.acc.y,
        }
    }

    /// Mutable reference to the x-dimension of this particle
    fn z_mut(&mut self) -> DimRefMut<'_> {
        DimRefMut {
            pos: &mut self.pos.z,
            vel: &mut self.vel.z,
            acc: &mut self.acc.z,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vec3 {
    fn manhattan_norm(self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

mod parse {
    use super::{Particle, Vec3};

    use nom::{
        bytes::complete::tag,
        character::complete::char,
        combinator::{eof, map},
        sequence::tuple,
        IResult,
    };

    use crate::lib::parse_i64 as parse_num;

    fn parse_vec(input: &str) -> IResult<&str, Vec3> {
        map(
            tuple((
                char('<'),
                parse_num,
                char(','),
                parse_num,
                char(','),
                parse_num,
                char('>'),
            )),
            |(_, x, _, y, _, z, _)| Vec3 { x, y, z },
        )(input)
    }

    fn parse_particle_line(input: &str) -> IResult<&str, Particle> {
        map(
            tuple((
                tag("p="),
                parse_vec,
                tag(", v="),
                parse_vec,
                tag(", a="),
                parse_vec,
                eof,
            )),
            |(_, pos, _, vel, _, acc, _)| Particle { pos, vel, acc },
        )(input)
    }

    pub(super) fn parse_input(input: &str) -> Vec<Particle> {
        let mut out = Vec::new();

        for line in input.lines() {
            let (_, particle) =
                parse_particle_line(line).expect("Line should parse, line not parse");
            out.push(particle);
        }

        out
    }
}

fn is_closer(a: Particle, b: Particle) -> bool {
    let a = a.normalize();
    let b = b.normalize();

    match a.acc.manhattan_norm().cmp(&b.acc.manhattan_norm()) {
        Ordering::Greater => return false,
        Ordering::Less => return true,
        Ordering::Equal => {}
    }

    // Basically acc dominates; that being equal, vel dominates; that being equal, pos dominates
    // I think this works (???) but I haven't actually worked out the math to make sure it works
    // in 3D (it definitely works in 1D)
    match (a.acc.manhattan_norm().cmp(&b.acc.manhattan_norm()))
        .then_with(|| a.vel.manhattan_norm().cmp(&b.vel.manhattan_norm()))
        .then_with(|| a.pos.manhattan_norm().cmp(&b.vel.manhattan_norm()))
    {
        Ordering::Greater => false,
        Ordering::Less => true,
        Ordering::Equal => panic!("I can't distinguish {:#?} from {:#?}", a, b),
    }
}

/// Returns Some(sqrt(n)), if that's an integer; otherwise, just returns None
fn int_sqrt(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }

    // Contract: low^2 <= n, high^2 > n
    let mut low = 0;
    let mut high = n + 1;

    while low + 1 < high {
        let mid = (low + high) / 2;
        if mid * mid > n {
            high = mid;
        } else {
            low = mid;
        }
    }

    if low * low == n {
        Some(low)
    } else {
        None
    }
}

fn int_div(num: i64, den: i64) -> Option<i64> {
    if den < 0 {
        int_div(-num, -den)
    } else if num < 0 {
        int_div(-num, den).map(|k| -k)
    } else if den == 0 {
        panic!("Cannot do modulus with den={}", den);
    } else if num % den == 0 {
        Some(num / den)
    } else {
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Collision {
    Discrete(Vec<i64>),
    Everywhere,
}

impl Collision {
    fn contains(&self, v: i64) -> bool {
        match self {
            Collision::Discrete(dat) => dat.contains(&v),
            Collision::Everywhere => true,
        }
    }
}

#[cfg(test)]
fn verify_intersects(mut a: Particle, mut b: Particle, t: usize) {
    let old_a = a;
    let old_b = b;
    let mut t0 = 0;
    while t0 < t {
        assert_ne!(a.pos, b.pos);
        a = a.advance();
        b = b.advance();
        t0 += 1;
    }
    assert_eq!(
        a.pos, b.pos,
        "At time {}, {} and {} did not match",
        t, old_a, old_b
    );
}

#[cfg(test)]
fn verify_intersects_1d(mut a: Dim, mut b: Dim, t: usize) {
    let old_a = a;
    let old_b = b;
    let mut t0 = 0;
    while t0 < t {
        assert_ne!(a.pos, b.pos);
        a = a.advance();
        b = b.advance();
        t0 += 1;
    }
    assert_eq!(
        a.pos, b.pos,
        "At time {}, {} and {} did not match",
        t, old_a, old_b
    );
}

fn verify_non_intersection(mut a: Particle, mut b: Particle, t_cutoff: usize) {
    let old_a = a;
    let old_b = b;
    for t in 0..t_cutoff {
        assert_ne!(
            a.pos, b.pos,
            "At time {}, {} and {} coincided, as {} and {}",
            t, old_a, old_b, a, b
        );
        a = a.advance();
        b = b.advance();
    }
}

fn intersects_1d(dim_a: Dim, dim_b: Dim) -> Collision {
    // need t where (apos[0] + avel[0] * t + aacc[0] * (t * t + t) / 2)
    // equals (bpos[0] + bvel[0] * t + bacc[0] * (t * t + t) / 2)
    // equiv: t^2 (aacc - bacc) + t (aacc - bacc + 2avel - 2bvel) + (2apos - 2bpos) == 0

    // Make those coefficients into the familiar binary formula ...
    let a = dim_a.acc - dim_b.acc;
    let b = dim_a.acc - dim_b.acc + 2 * (dim_a.vel - dim_b.vel);
    let c = 2 * (dim_a.pos - dim_b.pos);

    if a == 0 {
        if b == 0 {
            if c == 0 {
                Collision::Everywhere
            } else {
                Collision::Discrete(Vec::new())
            }
        } else {
            Collision::Discrete(int_div(-c, b).into_iter().filter(|n| *n >= 0).collect())
        }
    } else {
        // ... then do some intification of that formula
        if let Some(disc) = int_sqrt(b * b - 4 * a * c) {
            let mut out = Vec::with_capacity(2);
            int_div(-b - disc, 2 * a)
                .into_iter()
                .filter(|n| *n >= 0)
                .for_each(|i| out.push(i));
            int_div(-b + disc, 2 * a)
                .into_iter()
                .filter(|n| *n >= 0)
                .for_each(|i| out.push(i));
            Collision::Discrete(out)
        } else {
            Collision::Discrete(Vec::new())
        }
    }
}

fn intersects(a: Particle, b: Particle) -> Option<usize> {
    // pos[t] is pos[0] + vel[0] * t + acc[0] * (t * t + t) / 2
    // So need to find intersection points for a/b in all dimensions

    let x_int = intersects_1d(a.x(), b.x());
    let y_int = intersects_1d(a.y(), b.y());
    let z_int = intersects_1d(a.z(), b.z());

    let mut best: Option<i64> = None;

    match x_int {
        Collision::Discrete(x_int) => {
            for x_time in x_int {
                if y_int.contains(x_time) && z_int.contains(x_time) {
                    best = match best {
                        Some(old) => Some(old.min(x_time)),
                        None => Some(x_time),
                    };
                }
            }
        }
        Collision::Everywhere => match y_int {
            Collision::Discrete(y_int) => {
                for y_time in y_int {
                    if z_int.contains(y_time) {
                        best = match best {
                            Some(old) => Some(old.min(y_time)),
                            None => Some(y_time),
                        };
                    }
                }
            }
            Collision::Everywhere => match z_int {
                Collision::Discrete(z_int) => {
                    best = z_int.into_iter().min();
                }
                Collision::Everywhere => best = Some(0),
            },
        },
    }

    // guaranteed to be positive
    best.map(|b| b as usize)
}

fn run_20a_with_input(input: &str) -> usize {
    let particles = parse::parse_input(input);

    let mut best = particles[0];
    let mut best_ind = 0;

    for (i, p) in particles.iter().copied().enumerate().skip(1) {
        if is_closer(p, best) {
            best = p;
            best_ind = i;
        }
    }

    best_ind
}

pub fn run_20a() -> usize {
    run_20a_with_input(INPUT)
}

fn run_20b_with_input(input: &str) -> usize {
    let particles = parse::parse_input(input);
    let mut removed = vec![false; particles.len()];

    // This is O(n^3) in the worst case -- we iterate through pairs (n^2) and delete
    // anything involved which is tied for first collision, then just repeat
    // In practice I think it converges quickly, though?
    loop {
        // particleIdx -> earliest collision (if any)
        let mut removal_time = HashMap::new();
        let mut earliest_removal: Option<usize> = None;

        for i in (0..particles.len()).filter(|&i| !removed[i]) {
            for j in (i + 1..particles.len()).filter(|&j| !removed[j]) {
                if let Some(time) = intersects(particles[i], particles[j]) {
                    #[cfg(test)]
                    {
                        verify_intersects(particles[i], particles[j], time);
                    }

                    earliest_removal = earliest_removal
                        .map(|old_time| old_time.min(time))
                        .unwrap_or(time)
                        .into();

                    let i_entry = removal_time.entry(i).or_insert(time);
                    *i_entry = (*i_entry).min(time);

                    let j_entry = removal_time.entry(j).or_insert(time);
                    *j_entry = (*j_entry).min(time);
                } else {
                    verify_non_intersection(particles[i], particles[j], 100);
                }
            }
        }

        if let Some(time) = earliest_removal {
            for (i, t) in removal_time {
                if t == time {
                    removed[i] = true;
                }
            }
        } else {
            return removed.into_iter().filter(|t| !*t).count();
        }
    }
}

pub fn run_20b() -> usize {
    run_20b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part_str(input: &str) -> Particle {
        parse::parse_input(input)[0]
    }

    const SAMPLE_20A: &str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    const SAMPLE_20B: &str = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

    #[test]
    fn sample_20a() {
        assert_eq!(run_20a_with_input(SAMPLE_20A), 0);
    }

    #[test]
    fn sample_20b() {
        assert_eq!(run_20b_with_input(SAMPLE_20B), 1);
    }

    #[test]
    fn test_intersection() {
        let a = parse::parse_input("p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>")[0];
        let b = parse::parse_input("p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>")[0];
        let c = parse::parse_input("p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>")[0];
        let d = parse::parse_input("p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>")[0];

        assert_eq!(intersects(a, b), Some(2));
        assert_eq!(intersects(b, a), Some(2));
        assert_eq!(intersects(a, c), Some(2));
        assert_eq!(intersects(c, a), Some(2));
        assert_eq!(intersects(b, c), Some(2));
        assert_eq!(intersects(c, b), Some(2));

        assert_eq!(intersects(a, d), None);
        assert_eq!(intersects(b, d), None);
        assert_eq!(intersects(c, d), None);
        assert_eq!(intersects(d, a), None);
        assert_eq!(intersects(d, b), None);
        assert_eq!(intersects(d, c), None);

        assert_eq!(intersects(a, a), Some(0));
        assert_eq!(intersects(b, b), Some(0));
        assert_eq!(intersects(c, c), Some(0));
        assert_eq!(intersects(d, d), Some(0));
    }

    #[test]
    fn complex_intersection_1() {
        let a = part_str("p=<3090,1248,319>, v=<80,-33,13>, a=<-8,0,-1>");
        let b = part_str("p=<6288,-1092,-227>, v=<18,7,47>, a=<-9,1,-2>");

        assert_eq!(intersects(a, b), Some(39));
    }

    #[test]
    fn complex_intersection_2() {
        let a = part_str("p=<429,726,462>, v=<-36,-36,-19>, a=<-0,-6,-4>");
        let b = part_str("p=<1705,-165,1331>, v=<-134,9,-104>, a=<-3,0,-3>");

        assert_eq!(intersects(a, b), Some(11));

        verify_intersects(a, b, 11);
    }

    #[test]
    fn complex_intersection_3() {
        let a = part_str("p=<1500,413,-535>, v=<-119,22,36>, a=<-5,-12,3>");
        let b = part_str("p=<65,1223,-530>, v=<-14,-136,52>, a=<2,2,0>");

        verify_intersects(a, b, 10);
        assert_eq!(intersects(a, b), Some(10));
    }

    #[test]
    fn complex_intersection_4() {
        let a = part_str("p=<1556,2084,1247>, v=<-97,-115,-45>, a=<0,-2,-4>");
        let b = part_str("p=<-92,-212,615>, v=<23,-65,37>, a=<-2,9,-9>");

        verify_intersects(a, b, 16);

        assert_eq!(
            intersects_1d(a.x(), b.x()),
            Collision::Discrete(vec![16, 103])
        );

        // unverified
        assert_eq!(intersects_1d(a.y(), b.y()), Collision::Discrete(vec![16]));
        assert_eq!(intersects_1d(a.z(), b.z()), Collision::Discrete(vec![16]));

        assert_eq!(intersects(a, b), Some(16))
    }

    #[test]
    fn complex_1d() {
        let a = Dim {
            pos: 1247,
            vel: -45,
            acc: -4,
        };
        let b = Dim {
            pos: 615,
            vel: 37,
            acc: -9,
        };

        verify_intersects_1d(a, b, 16);

        assert_eq!(intersects_1d(a, b), Collision::Discrete(vec![16]));
    }
}
