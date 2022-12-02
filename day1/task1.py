with open('test.txt', 'r') as f:
    max_sum = 0
    cur_sum = 0
    for l in f.readlines():
        if len(l) == 1:
            max_sum = max(max_sum, cur_sum)
            cur_sum = 0
        else:
            cur_sum += int(l)
print(max_sum)