
if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        lines = f.readlines()
        items = []
        oper_type = []
        oper = []
        test = []
        if_true = []
        if_false = []
        count = []
        for i in range(0, len(lines), 7):
            items.append([])
            l = ''.join(lines[i + 1].split(','))
            tokens = [int(w) for w in (l.split()) if w.isnumeric()]
            for t in tokens:
                items[-1].append(t)
            oper_type.append('*' if '*' in lines[i + 2] else '+')
            if lines[i + 2].split()[-1] == 'old':
                oper.append('old')
            else: oper.append(int(lines[i + 2].split()[-1]))
            test.append(int(lines[i + 3].split()[-1]))
            if_true.append(int(lines[i + 4].split()[-1]))
            if_false.append(int(lines[i + 5].split()[-1]))
            count.append(0)
        mod = 1
        for t in test:
            mod *= t
        def turn(monkey: int):
            global items, oper, oper_type, test, if_true, if_false, count
            if len(items[monkey]) == 0:
                return
            n = len(items[monkey])
            for _ in range(n):
                item = items[monkey][0]
                count[monkey] += 1
                op = item if oper[monkey] == 'old' else oper[monkey]
                new_worry = item + op if oper_type[monkey] == '+' else item * op
                new_worry %= mod
                if new_worry % test[monkey] == 0:
                    items[if_true[monkey]].append(new_worry)
                else:
                    items[if_false[monkey]].append(new_worry)
                items[monkey] = items[monkey][1:]


        def round():
            n = len(items)
            for i in range(n):
                turn(i)
        
        for _ in range(10000):
            round()
        count.sort()
        print(count[-1] * count[-2])

            
