for i in range(3):
	print()
	for letter in "abcabcabcabc":
		if letter == "c":
			break
		print(str(letter), end="")
    
print()

for i in range(3):
	print("\nLoop at i = " + str(i))
	for letter in "abcabcabcabc":
		if letter == "c":
			continue
		print(str(letter), end="")
        
print()

while True:
	print()
	for letter in "defdefdef":
		if letter == "f":
			break
		print(str(letter), end="")
	break     

