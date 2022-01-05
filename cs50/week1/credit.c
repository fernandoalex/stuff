#include <stdio.h>

/* 
 * 4003600000000014
 */
int is_valid(long card_number) {
	int sum = 0;
	int digit = 0;
	int digit_times_2 = 0;

	while (card_number > 0) {
		sum += card_number % 10;

		card_number /= 10;
		digit = card_number % 10;
		digit_times_2 = digit * 2;

		while (digit_times_2 > 0) {
			sum += digit_times_2 % 10;
			digit_times_2 /= 10;
		}

		card_number /= 10;
	}

	if (sum % 10 == 0) {
		return 0;
	} else {
		return 1;
	}
}

// Get card provider
int get_provider(long card_number) {
	while (card_number > 100) {
		card_number /= 10;
	}

	return card_number;
}

// get long
long get_long(long number){
	return number;
}

int main() {
	// Lets pick a long here, but when doing a copy to the CS50 IDE
	// we need to change this to use the get_long function.
	int valid = 1;

	while (valid != 0)	{
		long card_number = get_long(4003600000000014);
		valid = is_valid(card_number);

		if (0 == valid) {
			if (
				get_provider(card_number) == 34 ||
				get_provider(card_number) == 37
			) {
				printf("AMEX");
			} 
			else if (
				get_provider(card_number) == 51 ||
				get_provider(card_number) == 52 ||
				get_provider(card_number) == 53 ||
				get_provider(card_number) == 54 ||
				get_provider(card_number) == 55
			) {
				printf("MASTERCARD");
			} 
			else if (
				(get_provider(card_number) / 10) == 4
			) {
				printf("VISA");
			} 
			else {
				printf("INVALID");
				valid = 1;
			}
		} else {
			printf("INVALID");
			continue;
		}
	}
}
