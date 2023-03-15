use clap::{Parser};

use bioshell_statistics::{Histogram, OnlineMultivariateStatistics};

use analid::{Grid, read_points};

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
    /// minimum number of observations per plot to print it; plots with fewer measurements will be discarded
    #[clap(short, long, default_value_t = 50, short='m')]
    min_observations: usize,
    /// percentile of observations to trim off; by default 5% observations from each plot are removed from both tails
    #[clap(short, long, default_value_t = 5.0, short='p')]
    percent: f64,
}

fn main() {

    let args = Args::parse();
    let points = read_points(&args.infile);
    let bin_width = args.bin_width;
    let grid: Grid = Grid::new(bin_width, bin_width, points);

    for key in grid.data().keys() {
        let n = grid.count_points(key);
        if n < args.min_observations { continue; }
        let i_from = (n as f64 * args.percent / 100.0) as usize;
        let i_to = (n - i_from)  as usize;

        for point in &grid.points(key).unwrap()[i_from..i_to] {
            println!("{}",point);
        }
    }
}
