#include <stdio.h>

int main(void) {
	int value = 1234;

	while (value > 0) {
		int digit = value % 10;
		value /= 10;
		printf("%d\n", digit);
	}
}
