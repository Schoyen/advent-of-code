import numpy as np
import numba

with open("dat/input.dat", "r") as f:
    dat = np.array([[ord(c) for c in line.strip()] for line in f])

@numba.njit
def find_diff_by_one(dat):
    len_string = len(dat[0])
    for i in range(len(dat)):
        for j in range(i + 1, len(dat)):

            sub = dat[i] - dat[j]

            if len(sub[sub == 0]) == (len_string - 1):
                return i, j

    return -1, -1


i, j = find_diff_by_one(dat)
diff = dat[i] - dat[j]

diff_index = np.argwhere(np.abs(diff) > 0)[0, 0]
answer = "".join([chr(c) for c in dat[i, :diff_index]]) + "".join(
    [chr(c) for c in dat[i, diff_index + 1 :]]
)

print ("Common letters:", answer)
