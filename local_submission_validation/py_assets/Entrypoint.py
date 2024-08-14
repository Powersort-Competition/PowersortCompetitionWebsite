import sys

import Powersort as Powersort
import Timsort as Timsort
import Counters as Counters

def cost(lst, sorter):
    wrapped = [Counters.ComparisonCounter(x) for x in lst]
    Counters.reset_counters()
    sorter.sort(wrapped)
    assert Counters.ComparisonCounter.EQ_COMPARISONS == 0

    return {
        'Algorithm': sorter.name(),
        'Comparisons': Counters.ComparisonCounter.COMPARISONS,
        'MergeCost': Counters.MergeCosts.MERGECOST
    }

def compare_sorters(lst, sorters = [Powersort, Timsort]):
    sorters = sorted(sorters, key = lambda sorter: sorter.name())

    return [cost(lst, sorter) for sorter in sorters]

with open(sys.argv[1], 'r') as file:
    print(compare_sorters(list(file.read())))

# stdout.write(compare_sorters([1, -1, 1990]))