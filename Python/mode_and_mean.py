import sys

from generators import generate_sorted_list
from stats.median import get_median
from stats.mode import get_mode

length = int(sys.argv[1])

list = generate_sorted_list(length)

print("sorted list: ", list)
print()

median = get_median(list)

print("median: ", median)

mode = get_mode(list)

print("mode: ", mode)
