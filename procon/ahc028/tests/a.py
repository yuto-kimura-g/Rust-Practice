for i in range(100):
    print(i, (i & (1 << 4)) == 0, (i & ((1 << 4)-1)) == 0)
