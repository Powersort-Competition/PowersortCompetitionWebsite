class ComparisonCounter:
    """
    Class that wraps an object and counts the number of comparisons made.
    """
    COMPARISONS = 0
    EQ_COMPARISONS = 0


    def __init__(self, obj):
        self.obj = obj


    def __lt__(self, other):
        ComparisonCounter.COMPARISONS += 1
        return self.obj < other.obj

    def __le__(self, other):
        ComparisonCounter.COMPARISONS += 1
        return self.obj <= other.obj

    def __gt__(self, other):
        ComparisonCounter.COMPARISONS += 1
        return self.obj > other.obj

    def __ge__(self, other):
        ComparisonCounter.COMPARISONS += 1
        return self.obj >= other.obj

    def __eq__(self, other):
        ComparisonCounter.EQ_COMPARISONS += 1
        return self.obj == other.obj

    def __ne__(self, other):
        ComparisonCounter.EQ_COMPARISONS += 1
        return self.obj != other.obj

    def __repr__(self):
        return 'ComparisonCounter(' + repr(self.obj) + ')'

    def __str__(self):
        return 'ComparisonCounter(' + str(self.obj) + ')'


class MergeCosts:

    MERGECOST = 0

    def __init__(self):
        pass




def reset_counters():
    ComparisonCounter.COMPARISONS = 0
    ComparisonCounter.EQ_COMPARISONS = 0
    MergeCosts.MERGECOST = 0

def print_counters():
    print('Comparisons:', ComparisonCounter.COMPARISONS)
    print('Eq Comparisons:', ComparisonCounter.EQ_COMPARISONS)
    print('MergeCosts:', MergeCosts.MERGECOST)


if __name__ == '__main__':
    import random
    A = list(range(10))
    random.shuffle(A)
    B = [ComparisonCounter(x) for x in A]
    print(B)
    reset_counters()
    B.sort()
    print(B)
    print_counters()