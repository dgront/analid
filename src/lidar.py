import sys
import matplotlib.pyplot as plt
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
for key in grid.keys():
    stats = grid.plot_statistics(key)
    i, j = stats.key
    avg[i][j] = stats.avg
    mode[i][j] = stats.mode
    counts[i][j] = stats.count

counts = np.array(counts)
avg = np.array(avg)
mode = np.array(mode)

plt.imshow(counts, cmap ='viridis')
plt.savefig("counts.png")

plt.imshow(avg, cmap ='viridis')
plt.savefig("avg.png")

plt.imshow(mode, cmap ='viridis')
plt.savefig("mode.png")


