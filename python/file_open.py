scores = []
highest_score = 0
result_f = open("results.txt")
for line in result_f:
        (juge, score) = line.split()
        scores.append(score)
        if score > highest_score:
                highest_score = score
result_f.close()
print("high score: %s" % highest_score)

scores.sort(reverse = True)

for i in range(0, 3):
    print(scores[i])
