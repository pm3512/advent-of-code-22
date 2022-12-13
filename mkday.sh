#!/usr/bin/bash

mkdir "day$1"
cd "day$1"
touch in1.txt
touch test1.txt
touch task1.py
touch task2.py
echo -e "import argparse\n" >> task1.py
echo "if __name__ == '__main__':" >> task1.py
echo "    parser = argparse.ArgumentParser(description='solution to AoC 22 day $1')" >> task1.py
echo "    parser.add_argument('file')" >> task1.py
echo "    with open(parser.parse_args().file, 'r') as f:" >> task1.py