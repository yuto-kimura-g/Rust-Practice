import heapq
N = int(input())
P = [list(map(int, input().split())) for _ in range(N)]
R = [list(map(int, input().split())) for _ in range(N)]
D = [list(map(int, input().split())) for _ in range(N - 1)]

pq = list()
heapq.heapify(pq)
# (action_cnt, money, y, x)
pq.append((0, 0, 0, 0))
INF = float("inf")
c = [[INF for _ in range(N)] for _ in range(N)]
while pq:
    action_cnt, money, y, x = heapq.heappop(pq)
    # print("c:", action_cnt, money, y, x)
    # for ci in c:
    #     print(ci)
    c[y][x] = min(c[y][x], action_cnt)
    if y == N - 1 and x == N - 1:
        break
    for dy, dx in ((0, 1), (1, 0), (0, 0)):
        ny, nx = y + dy, x + dx
        if not (0 <= ny < N and 0 <= nx < N):
            continue
        if (dy, dx) == (0, 0):
            t = INF
            if y < N - 1:
                t = min(t, D[y][x])
            if x < N - 1:
                t = min(t, R[y][x])
            if t == INF:
                t = 1
            heapq.heappush(pq, (action_cnt + 1 * t, money + P[y][x] * t, ny, nx))
        elif (dy, dx) == (0, 1):
            if x < N - 1 and money >= R[y][x]:
                heapq.heappush(pq, (action_cnt + 1, money - R[y][x], ny, nx))
        elif (dy, dx) == (1, 0):
            if y < N - 1 and money >= D[y][x]:
                heapq.heappush(pq, (action_cnt + 1, money - D[y][x], ny, nx))
        else:
            raise ValueError
print(c[N - 1][N - 1])
