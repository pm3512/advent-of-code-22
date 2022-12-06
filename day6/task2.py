if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        chars = f.readline()
        for i in range(len(chars) - 13):
            if len(set(chars[i:i+14])) == 14:
                print(i + 14)
                break