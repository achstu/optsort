class LinkedTree:

    class Node:
        def __init__(self, value):
            self.value = value
            self.childs = []

    def __init__(self, initial_state, isomorphism):
        self.root = self.Node(initial_state)
        self.isomorphism = isomorphism
        self.nodes = [self.root]
