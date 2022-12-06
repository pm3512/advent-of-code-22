if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        chars = f.readline()
        for i in range(len(chars) - 3):
            if len(set(chars[i:i+4])) == 4:
                print(i + 4)
                break