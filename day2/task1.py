if __name__ == '__main__':
    points = {
        'AX': 4,
        'AY': 8,
        'AZ': 3,
        'BX': 1,
        'BY': 5,
        'BZ': 9,
        'CX': 7,
        'CY': 2,
        'CZ': 6,
    }
    
    with open('in.txt', 'r') as f:
        score = 0
        for l in f.readlines():
            if len(l) > 1:
                score += points[''.join(l.split())]
    print(score)
