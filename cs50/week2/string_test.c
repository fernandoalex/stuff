#include <stdio.h>
#include <string.h>

int main (int argc, char *argv[]) {
	char *key = argv[1];

	printf("%s\n", key);

	printf("testing key\n");
	for(int i = 0; i < strlen(key); i++){
		printf("%c\n", key[i]); // remember that here we want to pring char so %c
	}

	printf("testing argv\n");
	for(int i = 0; i < strlen(argv[1]); i++){
		printf("%c\n", argv[1][i]);
	}
}
