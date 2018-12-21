import re
import os

import numpy as np

MAX_WIDTH = 0
MAX_HEIGHT = 0


class FabricClaim:
    def __init__(self, claim):
        parameters = re.findall(r"\d+", claim)

        self.id = int(parameters[0])
        self.cols = self._create_slice(parameters[1], parameters[3])
        self.rows = self._create_slice(parameters[2], parameters[4])

        global MAX_WIDTH, MAX_HEIGHT

        if self.cols.stop > MAX_WIDTH:
            MAX_WIDTH = self.cols.stop

        if self.rows.stop > MAX_HEIGHT:
            MAX_HEIGHT = self.rows.stop

    def _create_slice(self, start, num_steps):
        start = int(start)
        num_steps = int(num_steps)

        return slice(start, start + num_steps)

    @property
    def claim(self):
        return self.rows, self.cols


with open(os.path.join("dat", "input.dat"), "r") as f:
    claims = [FabricClaim(line) for line in f]


fabric = np.zeros((MAX_HEIGHT, MAX_WIDTH))

for c in claims:
    fabric[c.claim] += 1

print("Number of inches overlapping: {0}".format(np.sum(fabric > 1)))
