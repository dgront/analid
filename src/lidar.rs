use clap::{Parser};

use bioshell_statistics::{Histogram, OnlineMultivariateStatistics};

use analid::{Grid, PlotBounds, read_points, write_stats_for_bin};

#[derive(Parser, Debug)]
#[clap(name = "lidar")]
#[clap(version = "0.2")]
#[clap(about = "Simple analysis of LIDAR measurements", long_about = None)]
struct Args {
    /// staring conformation in the CSV format
    #[clap(short, long, default_value = "", short='f')]
    infile: String,
    /// temperature of the simulation
    #[clap(short, long, default_value_t = 10.0, short='w')]
    bin_width: f64,
}

fn main() {

    let args = Args::parse();
    let points = read_points(&args.infile);
    let grid: Grid = Grid::new(5.0, 5.0, points);
    let range = grid.bounds();
    // println!("{:?} -> {} x {}", range, range.width_x(),range.width_y());

    // ---------- print observations counts per box
    /*let key_by_size = largest_bins(&grid);
    for (key, size) in key_by_size {
            println!("{:?} : {}",key,size);
    }*/
    // ---------- print most probable height for each bin
    write_stats_for_bin(&grid);
}
