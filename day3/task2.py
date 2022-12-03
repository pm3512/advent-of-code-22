def priority(a: str):
    char = ord(a)
    return char - ord('a') + 1 if char >= ord('a') else char - ord('A') + 27

if __name__ == '__main__':
    priority_sum = 0
    with open('in2.txt', 'r') as f:
        lines = f.readlines()
        for i in range(0, len(lines), 3):
            l1, l2, l3 = lines[i], lines[i + 1], lines[i + 2]
            l1, l2, l3 = l1.strip(), l2.strip(), l3.strip()
            s1, s2, s3 = set(list(l1)), set(list(l2)), set(list(l3))
            for i in s1:
                if i in s2 and i in s3:
                    priority_sum += priority(i)
    print(priority_sum)