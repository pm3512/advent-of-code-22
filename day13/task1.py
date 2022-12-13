from ast import literal_eval
from functools import cmp_to_key

RIGHT = 1
WRONG = -1
INCONCLUSIVE = 0

if __name__ == '__main__':
    with open('in1.txt', 'r') as f:
        lines = f.readlines()
        def is_right_order(a, b):
            if isinstance(a, int) and isinstance(b, int):
                return RIGHT if a < b else WRONG if a > b else INCONCLUSIVE
            if isinstance(a, int):
                a = [a]
            if isinstance(b, int):
                b = [b]
            for (a_sub, b_sub) in zip(a, b, strict=False):
                right_sub = is_right_order(a_sub, b_sub)
                if right_sub != INCONCLUSIVE:
                    return right_sub
            a = len(a)
            b = len(b)
            return RIGHT if a < b else WRONG if a > b else INCONCLUSIVE


        signals_parsed = []
        for i in range(0, len(lines), 3):
            sig1 = literal_eval(lines[i])
            sig2 = literal_eval(lines[i + 1])
            signals_parsed += [sig1, sig2]
        signals_parsed += [[[2]], [[6]]]
            
        signals_parsed.sort(key=cmp_to_key(is_right_order), reverse=True)
        for i, s in enumerate(signals_parsed):
            if isinstance(s, list) and len(s) == 1 and isinstance(s[0], list) and len(s[0]) == 1:
                if s[0][0] == 2:
                    id1 = i + 1
                elif s[0][0] == 6:
                    id2 = i + 1
        print(id1 * id2)
