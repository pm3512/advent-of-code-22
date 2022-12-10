if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        x = 1
        cycle = 1
        ans = 0
        def wait_cycle():
            global cycle
            global ans
            global x
            if cycle % 40 == 20:
                ans += cycle * x
            cycle += 1
        for line in f.readlines():
            if line.startswith('noop'):
                wait_cycle()
            else:
                change = int(line.split()[1])
                wait_cycle()
                wait_cycle()
                x += change
        print(ans)
