def hyperoperation(a, n, b):
    if n == 3:
        return a ** b
    i = 0
    prev = a

    while i < b-1:
        prev = hyperoperation(a, n-1, prev)
        i += 1

    return prev

print(hyperoperation(3, 4, 3))