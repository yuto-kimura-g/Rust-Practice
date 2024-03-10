
class Node:
    def __init__(self, v: int) -> None:
        self.value = v
        self.prev = None
        self.next = None

    def __str__(self) -> str:
        return f"Node({self.value})"

    def __repr__(self) -> str:
        return f"Node({self.value})"


N = int(input())
A = list(map(int, input().split()))
p = dict()
V = Node(-1)
p[-1] = V
for a in A:
    n = Node(a)
    n.prev = V
    V.next = n
    p[a] = n
    V = V.next
Q = int(input())
for _ in range(Q):
    query = input()
    if query.startswith("1"):
        _, x, y = map(int, query.split())
        n = Node(y)
        l = p[x]
        if l.next is None:
            l.next = n
            n.prev = l
            p[l.value] = l
            p[n.value] = n
        else:
            r = l.next
            l.next = n
            n.prev = l
            r.prev = n
            n.next = r
            p[l.value] = l
            p[n.value] = n
            p[r.value] = r
    elif query.startswith("2"):
        _, x = map(int, query.split())
        n = p[x]
        if n.next is None:
            l = n.prev
            l.next = None
            p[x] = None
        else:
            l = n.prev
            r = n.next
            l.next = r
            r.prev = l
            p[x] = None
    else:
        raise ValueError
# print(p)
ans = list()
V = p[-1]
while True:
    if V is None:
        break
    # print(V)
    ans.append(V.value)
    V = V.next
print(*ans[1:])
