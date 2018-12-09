import re

start = []
end = []
task_dict = {}

with open("dat/input.dat", "r") as f:
    for line in f:
        _s, _e = re.findall(r"\b[A-Z]\b", line)

        if _e not in task_dict:
            task_dict[_e] = []
        task_dict[_e].append(_s)

        start.append(_s)
        end.append(_e)


def do_node(node):
    can_be_done = []

    for key in task_dict:
        if node in task_dict[key]:
            task_dict[key].remove(node)

        if len(task_dict[key]) == 0:
            can_be_done.append(key)

    return can_be_done


start_nodes = list(set(start) - set(end))
can_be_done = start_nodes
done = []

while len(can_be_done) != 0:
    node = sorted(can_be_done)[0]
    can_be_done.remove(node)
    done.append(node)
    can_be_done.extend(do_node(node))

    for _node in can_be_done:
        if _node in task_dict:
            del task_dict[_node]


print("Answer:", "".join(done))
