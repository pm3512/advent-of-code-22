import argparse

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='solution to AoC 22 day 14')
    parser.add_argument('file')
    with open(parser.parse_args().file, 'r') as f:
        lines = f.readlines()
        xmin = -1
        xmax = -1
        ymax = -1
        for l in lines: 
            for segment in l.split('->'):
                segment = segment.strip().split(',')
                x = int(segment[0])
                y = int(segment[1])
                if xmin < 0 or xmin > x:
                    xmin = x
                xmax = max(xmax, x)
                ymax = max(ymax, y)
        
        m = ymax + 1
        n = (xmax - xmin + 1)

        occupied = [[False for _ in range (n)] for _ in range(m)]

        for l in lines:
            l = l.split('->')
            start = l[0].strip().split(',')
            x = int(start[0]) - xmin
            y = int(start[1])
            for segment in l[1:]:
                segment = segment.strip().split(',')
                x_next = int(segment[0]) - xmin
                y_next = int(segment[1])
                assert x == x_next or y == y_next
                if x == x_next:
                    for i in range(min(y, y_next), max(y, y_next) + 1):
                        occupied[i][x] = True
                else:
                    for i in range(min(x, x_next), max(x, x_next) + 1):
                        occupied[y][i] = True
                x = x_next
                y = y_next
            
        path = [(0, 500 - xmin)]

        def drop_sand():
            global path, occupied
            while True:
                y, x = path[-1]
                if y >= m - 1:
                    return False
                if not occupied[y + 1][x]:
                    path.append((y + 1, x))
                    continue
                if x == 0:
                    return False
                if not occupied[y + 1][x - 1]:
                    path.append((y + 1, x - 1))
                    continue
                if x >= n - 1:
                    return False
                if not occupied[y + 1][x + 1]:
                    path.append((y + 1, x + 1))
                    continue
                else:
                    occupied[y][x] = True
                    path.pop()
                    return True
        
        count = 0
        while drop_sand():
            count += 1
        print(count)
