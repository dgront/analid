use std::collections::HashMap;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use flate2::read::GzDecoder;

use bioshell_statistics::{Histogram, OnlineMultivariateStatistics};

/// Single data point measured by a drone
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


/// 2D grid where all the measurements are split among small square plots
#[derive(Clone)]
pub struct Grid {
    dx: f64,
    dy: f64,
    data: HashMap<(i16,i16), Vec<Point>>,
    bounds: PlotBounds
}

impl Grid {

    pub fn new(dx: f64, dy: f64, points: Vec<Point>) -> Grid {
        let bounds = PlotBounds::new(&points);
        let mut g = Grid{dx, dy, data: HashMap::new(), bounds};
        g.insert_all(&points);
        return g;
    }

    /// Returns a hash pointing to the plot a given point should be assigned to
    /// # Arguments
    /// * `p` - a measured point
    pub fn hash(&self, p:&Point) -> (i16,i16) {
        let x = p.x - self.bounds.min_x;
        let y = p.y - self.bounds.min_y;
        let ix = (x/self.dx) as i16;
        let iy = (y/self.dy) as i16;

        return (ix,iy);
    }

    /// Immutable access to points measured for a given plot
    pub fn get_plot(&self, key: (i16, i16)) -> &Vec<Point> {
        let v: &Vec<Point> = match self.data.get(&key) {
            None => panic!("unknown key: {:?}.", key),
            Some(v) => {v}
        };
        return v;
    }

    /// Count points measured for a given plot
    ///
    /// A plot is identified by its hashing tuple
    pub fn count_points(&self, key: &(i16, i16)) -> usize {
        return match self.data.get(key) {
            None => {0}
            Some(v) => {v.len()}
        }
    }

    fn insert(&mut self, p: Point) {
        let key: (i16,i16) = self.hash(&p);
        if !self.data.contains_key(&key) {
            self.data.insert(key, vec![]);
        }
        let v: &mut Vec<Point> = match self.data.get_mut(&key) {
            None => panic!("unknown key: {:?}.", key),
            Some(v) => {v}
        };
        v.push(p);
    }

    fn insert_all(&mut self, points: &Vec<Point>) {
        for p in points {
            self.insert(p.clone());
        }
    }

    pub fn data(&self) -> &HashMap<(i16,i16), Vec<Point>> { &self.data }

    pub fn bounds(&self) -> &PlotBounds { &self.bounds }
}

/// Describe position of a single plot
#[derive(Debug, Clone)]
pub struct PlotBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64
}

impl PlotBounds {

    /// Create a bounding square for a given set of points
    pub fn new(points: &Vec<Point>) -> PlotBounds {
        let mut min_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_x = points[0].x;
        let mut max_y = points[0].y;
        for p in points {
            if p.x < min_x { min_x = p.x}
            if p.y < min_y { min_y = p.y}
            if p.x > max_x { max_x = p.x}
            if p.y > max_y { max_y = p.y}
        }
        return PlotBounds{
            min_x,
            min_y,
            max_x,
            max_y
        };
    }

    /// Width of this plot along X axis
    pub fn width_x(&self) -> f64 { self.max_x - self.min_x }

    /// Width of this plot along Y axis
    pub fn width_y(&self) -> f64 { self.max_y - self.min_y }
}

pub fn read_points(fname: &str) -> Vec<Point> {

    let mut out: Vec<Point> = vec![];

    let file = File::open(fname).unwrap();
    let file = BufReader::new(file);
    let d = GzDecoder::new(file);

    for line in BufReader::new(d).lines() {
        let l = line.expect("");
        let tokens: Vec<&str> = l.split(",").collect();
        if tokens.len() == 3 {
            let x = match tokens[0].parse::<f64>() {
                Ok(v) => {v}
                Err(_) => {continue}
            };
            let y = match tokens[1].parse::<f64>() {
                Ok(v) => {v}
                Err(_) => {continue}
            };
            let z = match tokens[2].parse::<f64>() {
                Ok(v) => {v}
                Err(_) => {continue}
            };
            let p = Point{x, y, z};
            out.push(p);
        }
    }

    return out;
}

/// Returns size for each plot of the given grid.
///
/// Returns a vector holding  `((i16, i16), usize)` data:
/// * `(i16, i16)` is the hash referring to a given plot from the grid
/// * `usize` is the size of that plot
/// Returned list of plots is already sorted by size, i.e. the largest plot is the first element of
/// the returned vector
pub fn plots_by_size(data: &Grid) -> Vec<((i16, i16), usize)> {

    let mut key_size: Vec<((i16,i16),usize)> = vec![];
    for key in data.data().keys() {
        key_size.push( (key.clone(), data.count_points(&*key)));
    }
    key_size.sort_by_key(|k| -1 * k.1 as i32);

    return key_size;
}

pub fn write_points(fname: String, points: &Vec<Point>) {
    let mut file =  File::create(fname).unwrap();
    for p in points {
        writeln!(file, "{} {} {}", p.x, p.y, p.z);
    }
}
/// writes most populated bin
// pub fn write_most_populated(n_boxes: usize, data: &Grid) {
//     let key_by_size = plots_by_size(&data);
//     for i in 0..n_boxes {
//         let key = key_by_size[i].0;
//         let size = key_by_size[i].1;
//         let points_subset = &data.data()[&key];
//         let r = PlotBounds::new(points_subset);
//         write_points(format!("{}-{}", key.0, key.1),points_subset);
//

pub fn write_stats_for_bin(data: &Grid) {

    for (key, points) in data.data().iter() {
        let mut stats = OnlineMultivariateStatistics::new(1);
        let mut h: Histogram = Histogram::by_bin_width(0.25);
        for p in points {
            h.insert(p.z);
            stats.accumulate_1d(p.z);
        }
        let (mi, ma, v) = h.mode();
        let mode: f64 = (mi + ma) / 2.0;
        println!("{:3} {:3}  {:4}  {:7.2} {:7.2} {:7.2}   {:7.2} {:6.4}", key.0, key.1,
                 stats.count(),  stats.min(0), stats.avg(0), stats.max(0), mode, v/h.sum());
    }
}
