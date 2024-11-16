import itertools
from ordering import PartialOrder


def search(order: PartialOrder):
    if order.count_permutations() == 1:
        return

    for i, j in itertools.combinations(range(order.size), 2):
        left = order.with_comparison(i, j)
        right = order.with_comparison(i, j)

        diff = abs(left.count_permutations() - right.count_permutations())

        if diff <= 1:
            print(left, right)


# empty = PartialOrder(3)
# search(empty)

a = PartialOrder(4).with_comparison(1, 0).with_comparison(1, 2)
b = PartialOrder(4).with_comparison(3, 0).with_comparison(3, 1)

print(a.isomorphism(b))

# print([x for x in itertools.combinations(range(3), 2)])


# from permutation import Permutation

# pi = Permutation([1, 3, 0, 2])
# print(pi.apply("UGAR"))

# print(pi.transform(0))
# print(pi.transform(1))
# print(pi.transform(2))
# print(pi.transform(3))
