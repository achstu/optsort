import itertools
from ordering import PartialOrder


def search(order: PartialOrder, depth=0, index=1):
    if order.count_permutations() == 1:
        return (order, depth, None, None, None)

    for i, j in itertools.combinations(range(order.size), 2):
        if not (order.consistent(i, j) and order.consistent(j, i)):
            continue

        left = order.with_comparison(i, j)
        right = order.with_comparison(j, i)

        diff = abs(left.count_permutations() - right.count_permutations())

        if diff <= 1:
            return (
                order,
                depth,
                (i, j),
                search(left, depth + 1, 2 * index),
                search(right, depth + 1, 2 * index + 1),
            )

    return None


root = PartialOrder(3)
tree = search(root)
print(tree)
