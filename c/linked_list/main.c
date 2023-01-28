#include <stdio.h>
#include <stdlib.h>

typedef struct {
	void *next;
	int data;
} Node;

Node *head = NULL;

Node *addNode(int data) {

	Node *new = NULL;

	// if the list is empty
	if (head == NULL) {
		new = malloc(sizeof(Node));
		if (new == NULL) return NULL;

		new->data = data;
		head = new;
		new->next = NULL;
	} else {
		new = malloc(sizeof(Node));
		if (new == NULL) return NULL;

		new->data = data;
		new->next = head;
		head = new;
	}

	return new;

}

Node *insertNode(int data, int position) {
	Node *current = head;
	while (current != NULL && position != 0) {
		position--;
	}

	if (position != 0) {
		printf("Requested position too far into the list.");
		return NULL;
	}

	Node *new = malloc(sizeof(Node));
	if (new == NULL) return NULL;

	new->data = data;
	new->next = current->next;
	current->next = new;

	return new;
}


int removeNode(int data) {
	Node *current = head;
	Node *prev = head;

	while (current != NULL) {
		if (current->data == data) {
			if (current == head) {
				head = current->next;
			} else {
				prev->next = current->next;
				free(current);
				current = NULL;
			}

		return 1;
		}
		prev = current;
		current = current->next;
	}

	return 0;
}

void printList() {
	Node *current = head;
	while (current != NULL) {
		printf("%d->", current->data);
		current = current->next;
	}

	printf("\n");
}

void printMenu() {
	printf("Options are:\n");
	printf("\t1. Add a node to the list.\n");
	printf("\t2. Remove{} a node from the list.\n");
	printf("\t3. Insert a node to the list.\n");
	printf("\t4. Print your list.\n");
	printf("\t5. Quit.\n");
	return;
}

int main (int argc, char **argv) {
	int option = -1;
	int arg = 0;
	int arg2 = 0;
	while (option != 4) {
		printMenu();
		int num_received = scanf("%d", &option);
		if (num_received == 1 && option > 0 && option <= 5) {
			switch(option) {
				case 1:
					printf("what to add:\n");
					scanf("%d", &arg);
					Node *new = addNode(arg);
					break;
				case 2:
					printf("what to remove:\n");
					scanf("%d", &arg);
					int success = removeNode(arg);
					if (!success) printf("Element not found.");
					break;
				case 3:
					printf("what to add:\n");
					scanf("%d", &arg);
					printf("what position:");
					scanf("%d", &arg2);
					new = insertNode(arg, arg2);
					if (new == NULL) printf("failed to insert");
					break;
				case 4:
					printList();
					break;
				case 5:
					break;
			}
		}
	}
	return 0;
}
