
hex_numbers = []
for i in range(10000):
    left_nibbles = '{:05x}'.format(random.randint(0, 10000))
    right_nibbles = ''.join(random.choices('0123456789abcdef', k=3))
    hex_number = left_nibbles + right_nibbles
    hex_numbers.append(hex_number)

print(hex_numbers)
