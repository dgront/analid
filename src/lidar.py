import sys
import matplotlib.pyplot as plt
from mpl_toolkits.axes_grid1 import make_axes_locatable
import numpy as np

import pyanalid

lidar_file = sys.argv[1]
pts = pyanalid.read_points(lidar_file)
grid = pyanalid.Grid(5.0, 5.0, pts)

x, y = [], []
for key in grid.keys():
    x.append(key[0])
    y.append(key[1])

xu = max(sorted(list(set(x)))) + 1
yu = max(sorted(list(set(y)))) + 1
counts = [[0 for i in range(yu)] for j in range(xu) ]
avg = [[0 for i in range(yu)] for j in range(xu) ]
mode = [[0 for i in range(yu)] for j in range(xu) ]
h = [[0 for i in range(yu)] for j in range(xu) ]
for key in grid.keys():
    stats = grid.plot_statistics(key)
    i, j = stats.key
    avg[i][j] = stats.avg
    mode[i][j] = stats.mode
    counts[i][j] = stats.count
    h[i][j] = stats.max - stats.min

counts = np.array(counts)
avg = np.array(avg)
mode = np.array(mode)

def make_map_plot(map_data, fname):
    ax = plt.subplot()
    im = ax.imshow(map_data, cmap ='viridis')
    divider = make_axes_locatable(ax)
    cax = divider.append_axes("right", size="5%", pad=0.05)
    plt.colorbar(im, cax=cax)
    plt.savefig(fname)
    plt.clf()


make_map_plot(avg, "avg.png")
make_map_plot(mode, "mode.png")
make_map_plot(counts, "counts.png")
make_map_plot(h, "h.png")


