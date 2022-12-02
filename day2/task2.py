if __name__ == '__main__':
    points = {
        'AX': 3,
        'AY': 4,
        'AZ': 8,
        'BX': 1,
        'BY': 5,
        'BZ': 9,
        'CX': 2,
        'CY': 6,
        'CZ': 7,
    }
    
    with open('in.txt', 'r') as f:
        score = 0
        for l in f.readlines():
            if len(l) > 1:
                score += points[''.join(l.split())]
    print(score)
