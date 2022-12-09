if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        visited = set()
        x = [0 for _ in range(10)]
        y = [0 for _ in range(10)]
        visited.add((0, 0))
        def get_step(par: int, cur: int):
                if abs(x[cur] - x[par]) == 2 and abs(y[cur] - y[par]) == 2:
                    x[cur] = (x[cur] + x[par]) // 2
                    y[cur] = (y[cur] + y[par]) // 2
                elif abs(x[cur] - x[par]) == 2:
                    x[cur] = (x[cur] + x[par]) // 2
                    y[cur] = y[par]
                elif abs(y[cur] - y[par]) == 2:
                    y[cur] = (y[cur] + y[par]) // 2
                    x[cur] = x[par]
                if cur == 9:
                    visited.add((x[cur], y[cur]))

        for l in f.readlines():
            dir = l[0]
            steps = int(l.split()[1])
            for s in range(steps):
                if dir == 'L': x[0] -= 1
                elif dir == 'R': x[0] += 1
                elif dir == 'U': y[0] += 1
                elif dir == 'D': y[0] -= 1
                for p in range(9):
                    get_step(p, p + 1)
    print(len(visited))

