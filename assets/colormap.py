import numpy as np
import matplotlib.pyplot as plt
from matplotlib.colors import LinearSegmentedColormap, ListedColormap
from matplotlib import colormaps
import json

def test1():
    # Step 1: Get the viridis colormap
    viridis_listed = plt.get_cmap('viridis')

    # Step 2: Extract the colors from the ListedColormap
    colors = viridis_listed.colors  # Get the colors as a list

    # Step 3: Create a LinearSegmentedColormap from these colors
    viridis_linear = LinearSegmentedColormap.from_list("viridis_linear", colors)

    # Test the new colormap
    fig, ax = plt.subplots(2, 1, figsize=(6, 4))
    x = np.linspace(0, 10, 100)
    y = np.sin(x)

    # Using the original viridis ListedColormap
    sc1 = ax[0].scatter(x, y, c=y, cmap=viridis_listed)
    fig.colorbar(sc1, ax=ax[0], orientation='horizontal')
    ax[0].set_title('ListedColormap (Viridis)')

    # Using the new viridis LinearSegmentedColormap
    sc2 = ax[1].scatter(x, y, c=y, cmap=viridis_linear)
    fig.colorbar(sc2, ax=ax[1], orientation='horizontal')
    ax[1].set_title('LinearSegmentedColormap from ListedColormap')

    plt.tight_layout()
    plt.show()


def test2():
    cm = colormaps['viridis']
    cm2 = cm.resampled(256)


    cmaps = dict()
    cmaps['name'] = 'viridis'
    cmaps['colors'] = [[0.0, 0.0, 0.0] for _ in range(256)]    

    for i in range(256):
        cmaps['colors'][i] = cm2.colors[i, 0:3].tolist()

    print(cm2.colors[0][0:3])

    with open('viridis.json', 'w') as f:
        json.dump(cmaps, f, indent=4)

def main():
    test2()

 






if __name__ == '__main__':
    main()