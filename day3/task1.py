def priority(a: str):
    char = ord(a)
    return char - ord('a') + 1 if char >= ord('a') else char - ord('A') + 27

if __name__ == '__main__':
    priority_sum = 0
    with open('in1.txt', 'r') as f:
        for l in f.readlines():
            l = l.strip()
            left = l[:len(l) // 2]
            right = l[len(l) // 2:]
            assert len(left) == len(right)
            counts_left = set()
            counts_right = set()
            for i, j in zip(left, right):
                assert max(ord(i), ord(j)) <= ord('z')
                assert min(ord(i), ord(j)) >= ord('A')
                counts_left.add(i)
                counts_right.add(j)
            for k in counts_left:
                if k in counts_right:
                    priority_sum += priority(k)
    print(priority_sum)