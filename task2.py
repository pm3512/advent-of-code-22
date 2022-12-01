
with open('in.txt', 'r') as f:
    sums = []
    cur_sum = 0
    for l in f.readlines():
        assert len(sums) <= 3
        if len(l) == 1:
            sums.append(cur_sum)
            if len(sums) > 3:
                sums = sorted(sums)[-3:]
            cur_sum = 0
        else:
            cur_sum += int(l)
    sums.append(cur_sum)
    if len(sums) > 3:
        sums = sorted(sums)[-3:]
print(sum(sorted(sums)))