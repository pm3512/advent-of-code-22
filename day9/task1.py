if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        visited = set()
        hx = hy = tx = ty = 0
        visited.add((tx, ty))
        for l in f.readlines():
            dir = l[0]
            steps = int(l.split()[1])
            for s in range(steps):
                if dir == 'L':
                    if tx > hx:
                        tx -= 1
                        ty = hy
                    hx -= 1
                elif dir == 'R':
                    if tx < hx:
                        tx += 1
                        ty = hy
                    hx += 1
                elif dir == 'U':
                    if ty < hy:
                        ty += 1
                        tx = hx
                    hy += 1
                else:
                    if ty > hy:
                        ty -= 1
                        tx = hx
                    hy -= 1
                visited.add((tx, ty))
    print(len(visited))

