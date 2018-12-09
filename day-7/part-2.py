import re

ZERO = ord("A") - 1

num_workers = 5
workers = {i: None for i in range(num_workers)}
for worker in workers:
    workers[worker] = {"task": None, "remaining": 0}

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

start_nodes = list(set(start) - set(end))
can_be_done = start_nodes
done = []


def distribute_work(workers, can_be_done):
    for w in workers:
        if len(can_be_done) == 0:
            break

        if workers[w]["task"] is None:
            workers[w]["task"] = sorted(can_be_done)[0]
            workers[w]["remaining"] = ord(workers[w]["task"]) - ZERO
            can_be_done.remove(workers[w]["task"])


def do_tick(workers, done):
    for w in workers:
        if workers[w]["task"] is not None:
            workers[w]["remaining"] -= 1

            if workers[w]["remaining"] == 0:
                done.append(workers[w]["task"])
                workers[w]["task"] = None


def remove_task(task_dict, done, can_be_done):
    for key in task_dict:
        for task in done:
            if task in task_dict[key]:
                task_dict[key].remove(task)
        if len(task_dict[key]) == 0:
            can_be_done.append(key)

    for node in can_be_done:
        if node in task_dict:
            del task_dict[node]


def work_to_be_done(task_dict, can_be_done, workers):
    if task_dict:
        return True

    if can_be_done:
        return True

    for w in workers:
        if workers[w]["task"] is not None:
            return True

    return False


tick_counter = 0

while work_to_be_done(task_dict, can_be_done, workers):
    print("Tick:", tick_counter)
    print("Tasks:", task_dict)
    print("Can be done:", can_be_done)
    for worker in workers:
        print("Worker {0}: {1}".format(worker, workers[worker]))
    print("Done:", done)
    print()
    remove_task(task_dict, done, can_be_done)
    distribute_work(workers, can_be_done)
    do_tick(workers, done)
    tick_counter += 1

print("Tick:", tick_counter)
print("Tasks:", task_dict)
print("Can be done:", can_be_done)
for worker in workers:
    print("Worker {0}: {1}".format(worker, workers[worker]))
print("Done:", done)
print()

print("Answer:", "".join(done))
