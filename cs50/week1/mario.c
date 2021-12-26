#include<stdio.h>

int main() {

	int height;

	do {
		printf("Height: ");
		scanf("%d", &height);
	} while (height <= 0 || height >= 9); 

	int i;
	for(i = 1;i <= height;i++) {

		int j;
		int full_height = height * 2;
		for(j = 1; j <= full_height + 1; j++){
				if(i + j <= height){
					printf(" ");
				} else {
					if(j == height + 1) {
					printf("  ");
				} else {
					if(j - i > height + 1) {
						printf("\n");
						break;
					} else {
						printf("#");
					}
				}
			}
		}
	}
	printf("\n");
}
