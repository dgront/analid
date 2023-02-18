import sys
import matplotlib.pyplot as plt
import numpy as np

import pyanalid

USAGE="""
Filter LIDAR observations

Removes all plots that have fewer than K observations, from all the remaining (populated) plots
removes P percentiles (given in percents, say 5) of observations on both tails of distribution

USAGE:
    python3 ./filter.py input-file K P

EXAMPLE:
    python3 ./filter.py lidar_data.csv.gz 50 5
"""

if len(sys.argv) == 1 or sys.argv[1] == "-h" or sys.argv[1] == "--help":
    print(USAGE)
    sys.exit(0)

lidar_file = sys.argv[1]
min_obs = int(sys.argv[2])
percent = float(sys.argv[3])

pts = pyanalid.read_points(lidar_file)
grid = pyanalid.Grid(5.0, 5.0, pts)

for key in grid.keys():
    n = grid.count_points(key)
    if n < min_obs: continue
    i_from = int(n*percent / 100.0)
    i_to = int(n - i_from)
    pts = grid.points(key)
    for point in pts[i_from:i_to]:
        print(point)


