import itertools
from ordering import PartialOrder


def search(order: PartialOrder):
    if order.count_permutations() == 1:
        return (order, 1, None, None, None)

    for i, j in itertools.combinations(range(order.size), 2):
        if not (order.consistent(i, j) and order.consistent(j, i)):
            continue

        left = order.with_comparison(i, j)
        right = order.with_comparison(j, i)

        diff = abs(left.count_permutations() - right.count_permutations())

        if diff <= 1:
            # isomorphism = left.isomorphism(right)
            # if isomorphism:
            # return order
            # else:
            return (
                order,
                1,
                (i, j),
                search(left),
                search(right),
            )

    return None


root = PartialOrder(3)
tree = search(root)
print(tree)
