if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        x = 1
        cycle = 0
        def wait_cycle():
            global cycle
            global ans
            global x
            if cycle % 40 == 0:
                print('')
                cycle = 0
            if abs(x - cycle) <= 1:
                print('#', end='')
            else:
                print('.', end='')
            cycle += 1
        for line in f.readlines():
            if line.startswith('noop'):
                wait_cycle()
            else:
                change = int(line.split()[1])
                wait_cycle()
                wait_cycle()
                x += change
        print('')
