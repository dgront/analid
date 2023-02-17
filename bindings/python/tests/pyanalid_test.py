import sys
print(sys.path)

import pyanalid

# ---------- Try to create a single point
p = pyanalid.Point(1.0, 2.0, 3.0)
print(p)

# ---------- Try reading points from a CSV file
pts = pyanalid.read_points("p100.csv")

grid = pyanalid.Grid(5.0, 5.0, pts)

bounds = grid.bounds()
print(f"{bounds.min_x} {bounds.max_x} {bounds.min_y} {bounds.max_y}")

for k in grid.keys():
    stats = grid.plot_statistics(k)
    print(stats)

