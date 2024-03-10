import random
import string
alpha = string.ascii_lowercase
N = 20
print(N)
for _ in range(N):
    s = ""
    for _ in range(2 * pow(10, 5) // N):
        i = random.randint(0, 25)
        s += alpha[i]
    print(s)
