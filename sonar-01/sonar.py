with open("sonar.input.txt", 'r') as fo:
    numbers = [int(x) for x in fo.read().split() if x]

test = [ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ]

def count_increases(x):
    z = zip(x, x[1:])
    increased = [(p,n) for p,n in z if n > p]
    return len(increased)

print("test:", count_increases(test))
print("real:", count_increases(numbers))
