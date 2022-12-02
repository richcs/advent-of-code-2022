
if __name__ == '__main__':
    file = open('input.txt', 'r')
    lines = file.readlines()
  
    scores_part1 = {
        "A X" : 4,
        "A Y" : 8,
        "A Z" : 3,
        "B X" : 1,
        "B Y" : 5,
        "B Z" : 9,
        "C X" : 7,
        "C Y" : 2,
        "C Z" : 6,
    }

    scores_part2 = {
        "A X" : 3,
        "A Y" : 4,
        "A Z" : 8,
        "B X" : 1,
        "B Y" : 5,
        "B Z" : 9,
        "C X" : 2,
        "C Y" : 6,
        "C Z" : 7,
    }

    total_part1 = 0
    total_part2 = 0
    for line in lines:
        line = line.strip()
        total_part1 += scores_part1[line]
        total_part2 += scores_part2[line]

    print(total_part1)
    print(total_part2)