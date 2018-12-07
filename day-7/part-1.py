
glob_dict = {}
with open("dat/input.dat", "r") as f:
    for line in f:
        line_list = line.strip().split()
        link = []
        for elem in line_list:
            if len(elem) == 1:
                link.append(elem)

        if link[-1] in glob_dict:
            glob_dict[link[-1]].append(link[0])
        else:
            glob_dict[link[-1]] = [link[0]]

print (glob_dict)
print (sorted(glob_dict))

rhs = set(glob_dict)
lhs = set([elem for elems in glob_dict.values() for elem in elems])
end_node = list(rhs - lhs)[0]

def find_path(key, node_str=""):
    if key in node_str:
        return node_str

    if key not in glob_dict:
        return node_str + key

    neighbors = glob_dict[key]

    for n_key in sorted(neighbors):
        node_str = find_path(n_key, node_str=node_str)

    return node_str + key
