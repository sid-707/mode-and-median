import random


def generate_sorted_list(length):
    list = []
    for _ in range(length):
        list.append(random.randint(0, 9))

    list.sort()

    return list
