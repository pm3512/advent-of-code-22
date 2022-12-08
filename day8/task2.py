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
        def get_score(i: int, j: int) -> int:
            l = j - 1
            while l >= 0 and grid[i][l] < grid[i][j]:
                l -= 1
            lscore = j if l < 0 else j - l
            r = j + 1
            while r < n and grid[i][r] < grid[i][j]:
                r += 1
            rscore = n - 1 - j if r > n - 1 else r - j
            u = i - 1
            while u >= 0 and grid[u][j] < grid[i][j]:
                u -= 1
            uscore = i if u < 0 else i - u
            d = i + 1
            while d < m and grid[d][j] < grid[i][j]:
                d += 1
            dscore = m - 1 - i if d > m - 1 else d - i
            return lscore * rscore * uscore * dscore
        
        maxscore = 0
        for i in range(m):
            for j in range(n):
                maxscore = max(maxscore, get_score(i, j))
        print(maxscore)
        