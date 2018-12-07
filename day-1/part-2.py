import numpy as np
import numba

dat = np.loadtxt("dat/input.dat").astype(int)

@numba.njit
def find_repeating_frequency(dat):
    encountered_frequencies = []
    i = 0
    freq = dat[i]
    i += 1

    while freq not in encountered_frequencies:
        encountered_frequencies.append(freq)

        freq += dat[i]
        i += 1

        if i == len(dat):
            i = 0

    return freq

print (find_repeating_frequency(dat))
