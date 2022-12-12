from collections import deque

if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        heights = []
        for i, l in enumerate(f.readlines()):
            heights.append([])
            for j, c in enumerate(l.strip()):
                if c == 'S':
                    heights[-1].append(0)
                    start = (i, j)
                elif c == 'E':
                    heights[-1].append(ord('z') - ord('a'))
                    end = (i, j)
                else:
                    heights[-1].append(ord(c) - ord('a'))
        m = len(heights)
        n = len(heights[0])
        dist = [[-1 for _ in range(n)] for _ in range(m)]
        dist[end[0]][end[1]] = 0
        q = deque([end])

        while True:
            y, x = q.popleft()
            h = heights[y][x]
            d = dist[y][x]
            if h == 0:
                print(d)
                break
            if y > 0 and heights[y - 1][x] - heights[y][x] >= -1 and dist[y - 1][x] == -1:
                q.append((y - 1, x))
                dist[y - 1][x] = d + 1
            if y < m - 1 and heights[y + 1][x] - heights[y][x] >= -1 and dist[y + 1][x] == -1:
                q.append((y + 1, x))
                dist[y + 1][x] = d + 1
            if x > 0 and heights[y][x - 1] - heights[y][x] >= -1 and dist[y][x - 1] == -1:
                q.append((y, x - 1))
                dist[y][x - 1] = d + 1
            if x < n - 1 and heights[y][x + 1] - heights[y][x] >= -1 and dist[y][x + 1] == -1:
                q.append((y, x + 1))
                dist[y][x + 1] = d + 1