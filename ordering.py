import itertools
import copy


def cmp(a, b):
    return (a > b) - (a < b)


class PartialOrder:
    def __init__(self, size):
        self.size: int = size
        self.childs: dict[int, set] = {i: set() for i in range(size)}

    def __repr__(self):
        return f"{self.childs}"

    def is_ancestor(self, root, node):
        return any(c == node or self.is_ancestor(c, node) for c in self.childs[root])

    def matches_permutation(self, perm):
        return not any(
            self.is_ancestor(perm[i], perm[j])
            for i, j in itertools.combinations(range(self.size), 2)
        )

    def count_permutations(self):
        return sum(
            1
            for perm in itertools.permutations(range(self.size))
            if self.matches_permutation(perm)
        )

    def with_comparison(self, i, j):
        # a[i] < a[j]
        result = copy.deepcopy(self)
        if not self.is_ancestor(j, i):
            result.childs[j].add(i)
        return result

    def with_permutation(self, perm):
        result = copy.deepcopy(self)
        result.childs = {
            perm[i]: set(perm[c] for c in self.childs[i]) for i in range(self.size)
        }
        return result

    # jaką permutacje musimy przyłożyć do other, żeby był równy self
    def isomorphism(self, other):
        for perm in itertools.permutations(range(self.size)):
            if self == other.with_permutation(perm):
                return perm

        return None

    def __eq__(self, other):
        return self.childs == other.childs
