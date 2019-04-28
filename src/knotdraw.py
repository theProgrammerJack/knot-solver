import pygame
from math import pi
from orderedset import OrderedSet

abc = "abcdefghijklmnopqrstuvwxyz"
a = list(abc)


knot = input("knot?")
setk = OrderedSet(sorted(knot.lower()))

setk = a[:a.index(setk[len(setk)-1])+1]

print(setk)

#setk = a[:(a.index()+1)]

knotfield = list()

m = 0
for c in setk:
	knotfield.append([None]*50)
	knotfield[m][0]= ord(c)
	m+=1

def column(letter):
	for i in range(0, len(knotfield)):
		if (ord(letter.lower())) == knotfield[i][0]:
			return i

			
def isRowEmpty(i):
	for j in range(0, len(knotfield)):
		if knotfield[j][i] != None:
			return False
	return True
			
def earliestEmptyRow():
	early = 1999
	for ahh in range(0, len(knotfield[0])):
		if isRowEmpty(ahh) and ahh < early:
			early = ahh
	return early
	
for c in knot:
		col = column(c)
		for i in range(1, len(knotfield[col])):
			if((knotfield[col][i] == None) and (col != 0 and knotfield[col-1][i] != None) or (col != len(knotfield)-1 and knotfield[col+1][i] != None)): 
				knotfield[col][i] = 1
			if(knotfield[col][i] == None and (col == 0 or knotfield[col-1][i] == None) and (col == len(knotfield)-1 or knotfield[col+1][i] == None)): 
				knotfield[col][i] = c
				break

if earliestEmptyRow() != 1999:	
	for i in range(0, len(knotfield)):
		knotfield[i] = knotfield[i][:earliestEmptyRow()+1]
		
		
print(knotfield)

pygame.init()
 
# Define the colors we will use in RGB format
BLACK = (  0,   0,   0)
WHITE = (255, 255, 255)
BLUE =  (  0,   0, 255)
GREEN = (  0, 255,   0)
RED =   (255,   0,   0)
 
# Set the height and width of the screen
size = [500, 500]
screen = pygame.display.set_mode(size)
 
pygame.display.set_caption(knot)
 
#Loop until the user clicks the close button.
done = False
clock = pygame.time.Clock()
 
spacingx = size[0] / (len(knotfield)+2)
spacingy = size[1] / len(knotfield[0])
while not done:
	clock.tick(10)
	for event in pygame.event.get(): 
		if event.type == pygame.QUIT:
			done=True 
 
  
	# Clear the screen and set the screen background
	screen.fill(WHITE)

	for i in range(0, len(knotfield)):
		for j in range(1, len(knotfield[0])):
			if knotfield[i][j] == None or knotfield[i][j] == 1:
			
				if i == len(knotfield)-1 or knotfield[i+1][j] == None or knotfield[i+1][j] == 1:
					pygame.draw.line(screen, (0, 0, 0), [spacingx*(i+2), spacingy*j], [spacingx*(i+2), spacingy*(j+1)], 3)
					
				if i == 0 or knotfield[i-1][j] == None or knotfield[i-1][j] == 1:
					pygame.draw.line(screen, (0, 0, 0), [spacingx*(i+1), spacingy*j], [spacingx*(i+1), spacingy*(j+1)], 3)
			else:
				if knotfield[i][j].isupper():
					pygame.draw.line(screen, (0,0,0), [spacingx*(i+2), spacingy*j], [spacingx*(i+1), spacingy*(j+1)], 3)
					pygame.draw.line(screen, (255,0,0), [spacingx*(i+1), spacingy*j], [spacingx*(i+2), spacingy*(j+1)], 3) 
				if knotfield[i][j].islower():
					pygame.draw.line(screen, (0,0,0), [spacingx*(i+1), spacingy*j], [spacingx*(i+2), spacingy*(j+1)], 3)
					pygame.draw.line(screen, (255,0,0), [spacingx*(i+2), spacingy*j], [spacingx*(i+1), spacingy*(j+1)], 3)					
					
				

	pygame.display.flip()
 

pygame.quit()