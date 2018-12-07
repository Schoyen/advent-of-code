
with open("dat/input.dat", "r") as f:
    dat = [line.strip() for line in f]

two_occ = 0
three_occ = 0

for line in dat:
    chars = set(line)

    _two = False
    _three = False
    for char in chars:
        if line.count(char) == 2:
            _two = True
            continue

        if line.count(char) == 3:
            _three = True

    two_occ += _two
    three_occ += _three

print ("Checksum: {0}".format(two_occ * three_occ))
