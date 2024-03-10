from atcoder.segtree import SegTree

N, Q = map(int, input().split())
A = list(map(int, input().split()))
seg = SegTree(max, -1, A)
for _ in range(Q):
    query = input()
    if query.startswith("1"):
        _, p, x = map(int, query.split())
        p -= 1  # 0-indexed
        seg.set(p, x)
        print(seg._d[seg._size:])
    elif query.startswith("2"):
        _, l, r = map(int, query.split())
        l, r = l - 1, r - 1  # 0-indexed
        largest = seg.prod(l, r)
        print(seg._d[seg._size:], l, r, largest)

        def f(x) -> bool:
            if x == -1:
                return True
            if not (l <= x <= r):
                return False
            return x < largest

        ans = seg.max_right(l, f)
        print(ans)
    else:
        raise ValueError("Invalid query")
