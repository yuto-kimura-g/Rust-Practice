import sortedcontainers

N, T = map(int, input().split())
AB = [tuple(map(int, input().split())) for _ in range(T)]
players = [0 for _ in range(N)]
scores = sortedcontainers.SortedDict()
scores[0] = set()
for i in range(N):
    scores[0].add(i)
for a, b in AB:
    a -= 1  # 0-indexed
    cur_score = players[a]
    new_score = cur_score + b
    players[a] = new_score
    scores[cur_score].remove(a)
    if len(scores[cur_score]) == 0:
        scores.pop(cur_score)
    if new_score in scores:
        scores[new_score].add(a)
    else:
        scores[new_score] = {a}
    print(len(scores))
