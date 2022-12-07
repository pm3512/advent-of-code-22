from collections import defaultdict

if __name__ == '__main__':
    dirs = set()
    tot_sizes = {}
    own_sizes = defaultdict(int)
    parents = {}
    children = defaultdict(list)
    with open('in1.txt', 'r') as f:
        lines = f.readlines()
    p = 0
    cur_dir = ''
    while p < len(lines):
        l = lines[p].strip()
        assert l.startswith('$')
        if l.startswith('$ cd'):
            if l.startswith('$ cd ..'):
                cur_dir = parents[cur_dir]
            elif l.startswith('$ cd /'):
                cur_dir = ''
            else:
                cur_dir = cur_dir + '/' + l[5:]
            dirs.add(cur_dir)
            p += 1
        elif l.startswith('$ ls'):
            p2 = p + 1
            while p2 < len(lines) and not lines[p2].startswith('$'):
                p2 += 1
            for j in range(p + 1, p2):
                l2 = lines[j].strip()
                if l2.startswith('dir'):
                    dirname = cur_dir + '/' + l2[4:]
                    parents[dirname] = cur_dir
                    children[cur_dir].append(dirname)
                else:
                    size = int(l2.split()[0])
                    own_sizes[cur_dir] += size
            p = p2

    leaves = [dir for dir in dirs if len(children[dir]) == 0]
    for l in leaves:
        tot_sizes[l] = own_sizes.get(l, 0)

    def get_size(dir: str):
        if dir in tot_sizes:
            return tot_sizes[dir]
        size = 0
        for child in children[dir]:
            size += get_size(child)
        size += own_sizes[dir]
        tot_sizes[dir] = size
        return size
    
    get_size('')

    sizes_sorted = sorted(list(tot_sizes.values()))
    tot_space = 70_000_000
    need_free = 30_000_000
    free_space = tot_space - tot_sizes['']
    to_free = need_free - free_space
    for s in sizes_sorted:
        if s >= to_free:
            print(s)
            break