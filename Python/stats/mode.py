def get_mode(list):
    counts_dict = dict()

    for i in list:
        if (i in counts_dict):
            counts_dict[i] += 1
        else:
            counts_dict[i] = 1

    return max(counts_dict, key=counts_dict.get)
