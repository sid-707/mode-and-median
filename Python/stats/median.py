def get_median(list):
    length = len(list)

    if (length % 2 == 0):
        middle_index = (length // 2) - 1
        middle_number = list[middle_index]
        next_middle_number = list[middle_index + 1]

        return ((middle_number + next_middle_number) / 2)

    middle_index = (length - 1) // 2

    return list[middle_index]
