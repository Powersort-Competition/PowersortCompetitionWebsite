import argparse
import json

import py_assets.Timsort as Timsort
import py_assets.Powersort as Powersort
import py_assets.Counters as Counters


parser = argparse.ArgumentParser(
        description = "Compute statistics for an input array for track A."
        )
parser.add_argument("FilePath", metavar = "F", type = str,
                    nargs = 1, help = "Input array txt file.")

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

def main():
    args = parser.parse_args()
    file_path = args.FilePath[0]
    
    with open(file_path, 'r') as file:
        lst = json.loads(file.read())
        print(compare_sorters(lst))


if __name__ == "__main__":
    main()
