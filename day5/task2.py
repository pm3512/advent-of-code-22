if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        lines = f.readlines()
        num_stacks = len(lines[0]) // 4
        stacks = [[] for _ in range(num_stacks)]
        command_start_line = 0
        for l in lines:
            if '1' in l:
                break
            command_start_line += 1
            for i in range(1, len(l), 4):
                if l[i].isalpha():
                    stacks[i // 4].append(l[i])
        stacks = list(map(lambda x: x[::-1], stacks))

        command_start_line += 2
        for (i, l) in enumerate(lines[command_start_line:]):
            nums = list(filter(lambda x: x.isnumeric(), l.split()))
            nums = list(map(int, nums))
            assert len(nums) == 3
            nums[1] -= 1
            nums[2] -= 1
            to_move = stacks[nums[1]][-nums[0]:]
            stacks[nums[2]] += to_move
            stacks[nums[1]] = stacks[nums[1]][:-nums[0]]
        
    print(stacks)
    print(list(stack[-1] for stack in stacks))
        
