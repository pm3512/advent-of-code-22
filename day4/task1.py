def get_sections(s: str):
    sec = s.split('-')
    return (int(sec[0]), int(sec[1]))

def is_overlapping(l: str):
    l = l.strip().split(',')
    (l1, r1) = get_sections(l[0])
    (l2, r2) = get_sections(l[1])
    return (l1 <= l2 and r2 <= r1) or (l2 <= l1 and r1 <= r2)

def main():
    with open('in1.txt', 'r') as f:
        count_overlap = 0
        for l in f.readlines():
            count_overlap += is_overlapping(l)
    print(count_overlap)

if __name__ == '__main__':
    main()