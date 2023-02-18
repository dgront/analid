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
    /// plot size in meters
    #[clap(short, long, default_value_t = 5.0, short='w')]
    bin_width: f64,
}

fn main() {

    let args = Args::parse();
    let points = read_points(&args.infile);
    let grid: Grid = Grid::new(5.0, 5.0, points);
    let range = grid.bounds();

    for (k, _v) in grid.data() {
        let stats = grid.plot_statistics(k);
        println!("{:3} {:3}  {:4}  {:7.2} {:7.2} {:7.2} {:7.2}", k.0, k.1,
                 stats.count,  stats.min, stats.avg, stats.max, stats.mode);
    }
}
