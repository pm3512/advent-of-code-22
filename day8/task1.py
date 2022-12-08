if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        visible = {l: set() for l in 'LRUD'}
        grid = []
        for l in f.readlines():
            l = list(l.strip())
            l = list(map(int, l))
            grid.append(l)
        m = len(grid)
        n = len(grid[0])
        for i, l in enumerate(grid):
            maxheight = -1
            for j, tree in enumerate(l):
                if tree > maxheight:
                    visible['L'].add((i, j))
                    maxheight = tree
            maxheight = -1
            for j, tree in enumerate(l[::-1]):
                if tree > maxheight:
                    visible['R'].add((i, n - j - 1))
                    maxheight = tree

        grid_t = [[None for _ in range(m)] for _ in range(n)]
        for i in range(n):
            for j in range(m):
                grid_t[i][j] = grid[j][i]

        for i, l in enumerate(grid_t):
            maxheight = -1
            for j, tree in enumerate(l):
                if tree > maxheight:
                    visible['U'].add((j, i))
                    maxheight = tree
            maxheight = -1
            for j, tree in enumerate(l[::-1]):
                if tree > maxheight:
                    visible['D'].add((m - j - 1, i))
                    maxheight = tree
        print(len(set().union(*[visible[l] for l in 'LRUD'])))
        