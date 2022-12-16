import argparse
from collections import deque
from tqdm import tqdm

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='solution to AoC 22 day 16')
    parser.add_argument('file', default='in1.txt')
    with open(parser.parse_args().file, 'r') as f:
        lines = f.readlines()
        flow = {}
        g = {}
        for l in lines:
            l = l.strip().replace(',', ' ').replace('=', ' ').replace(';', ' ').split()
            name = l[1]
            flow[name] = int(l[5])
            g[name] = set(l[10:])
        
        dists = {node: {} for node in g}

        for node in dists:
            q = deque([(node, 0)])
            while len(q) > 0:
                v, d = q.popleft()
                if v in dists[node]:
                    continue
                dists[node][v] = d
                for n in g[v]:
                    q.append((n, d + 1))
        
        dists = {v: dists[v] for v in dists if flow[v] > 0 or v == 'AA'}
        map_to_num = {v: i for i, v in enumerate(dists)}
        for v in dists:
            dists[v] = {map_to_num[u]: dists[v][u] for u in dists[v] if flow[u] > 0}
        dists = {map_to_num[v]: dists[v] for v in dists}
        flow = [flow[v] for v in flow if v in map_to_num]
        map_to_num = {i: v for i, v in enumerate(dists)}
        
        def maxflow(v: str, visited: set, t_left: int, rate: int, tot_flow: int, mask: int):
            sub = []
            visited.add(v)
            rate += flow[v]
            for u, d in dists[v].items():
                assert u > 0
                if u in visited or (mask & (1 << (u - 1))) == 0:
                    continue
                if d + 1 <= t_left:
                    sub.append(maxflow(u, visited, t_left - d - 1, rate, tot_flow + (d + 1) * rate, mask))
                    visited.remove(u)
            if len(sub) == 0:
                return tot_flow + t_left * rate
            else:
                return max(sub)

        maxnum = 1 << (len(dists) - 2)
        best = 0
        for mask in tqdm(range(maxnum)):
            cur = maxflow(0, set(), 26, 0, 0, mask) + maxflow(0, set(), 26, 0, 0, maxnum - mask)
            best = max(best, cur)
        print(best)
        
