highest_score = 0
result_f = open("results.txt")
for line in result_f:
        (juge, score) = line.split()
        if score > highest_score:
                highest_score = score
result_f.close()
print("high score:")
print(highest_score)
