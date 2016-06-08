#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

void die(const char *message){
        if(errno) {
                perror(message);
        } else {
                printf("ERROR: %s\n", message);
        }

        exit(1);
}

// Function Pointer "(*compare_cb)"
// typedef is used here to create a new type, like int or long
typedef int (*compare_cb)(int a, int b);

/**
Bubble sort funcion it uses compare_cb to sort
**/
int *bubble_sort(int *numbers, int count, compare_cb cmp){ //compare_cb is used to define cmp, remember that we used typedef to create a new type
        int temp = 0;
        int i = 0;
        int j = 0;
        int *target = malloc(count * sizeof(int));

        if(!target) die("Memory error.");

        //memcpy copies part of memory to other part of memory
        memcpy(target, numbers, count * sizeof(int));

        for(i = 0; i < count; i++){
                for(j = 0; j < count - 1; j++){
                        if(cmp(target[j], target[j+1]) > 0){
                                temp = target[j+1];
                                target[j+1] = target[j];
                                target[j] = temp;
                        }
                }
        }

        return target;
}

int sorted_order(int a, int b) {
        return a - b;
}

int reverse_order(int a, int b) {
        return b - a;
}

int strange_order(int a, int b) {
        if (a == 0 || b == 0) {
                return 0;
        } else {
                return a % b;
        }
}

/**
Used to test what we are sorting correctly
bt doing the sort and printing it out
**/
void test_sorting(int *numbers, int count, compare_cb cmp) {
        int i = 0;
        int *sorted = bubble_sort(numbers, count, cmp);

        if(!sorted) die("Failed to sort as requested.");

        for(i = 0; i < count; i++){
                printf("%d ", sorted[i]);
        }

        printf("\n");

        free(sorted);
}

int main(int argc, char *argv[]){
        if(argc < 2) die("USAGE: ex18 5 3 6 1 2");

        int count = argc - 1;
        int i = 0;
        char **inputs = argv + 1;

        int *numbers = malloc(count * sizeof(int));
        if(!numbers) die("Memory Error.");

        for(i = 0; i < count; i++) {
                numbers[i] = atoi(inputs[i]);
        }

        test_sorting(numbers, count, sorted_order);
        test_sorting(numbers, count, reverse_order);
        test_sorting(numbers, count, strange_order);

        return 0;
}
