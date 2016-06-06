#include <stdio.h>

//verify assumptions made by the program and print a diagnostic message if this assumption is false
#include <assert.h>

#include <stdlib.h>
#include <string.h>

struct Person {
        char *name;
        int age;
        int height;
        int weight;
};

struct Person *Person_create(char *name, int age, int height, int weight) {
        struct Person *who = malloc(sizeof(struct Person)); //memory allocate raw piece of memory in the size of Person
        assert(who != NULL); //basically checking that malloc didn't return a NULL invalid pointer

        who->name = strdup(name); //string duplicate
        who->age = age;
        who->height = height;
        who->weight = weight;

        return who;
}

void Person_destroy(struct Person *who) {
        assert(who != NULL);

        free(who->name);
        free(who);
}

void Person_print(struct Person *who) {
        printf("Name: %s\n", who->name);
        printf("\tAge: %d\n", who->age);
        printf("\tHeight: %d\n", who->height);
        printf("\tWeight: %d\n", who->weight);
}

int main(int argc, char *argv[]){
        //make two people structures
        struct Person *joe = Person_create("Joe Alex", 32, 64, 140);
        struct Person *frank = Person_create("Frank Blank", 20, 72, 180);

        printf("Joe is at memory location %p:\n", joe); //%p print out a pointe
        Person_print(joe);

        printf("Frank is at memory location %p:\n", frank); //%p print out a pointe
        Person_print(frank);

        // make everyone age 20 years and print them again
        joe->age += 20;
        joe->height -= 2;
        joe->weight += 40;
        Person_print(joe);

        frank->age += 20;
        frank->weight += 20;
        Person_print(frank);

        // destroy them both so we clean up
        Person_destroy(joe);
        Person_destroy(frank);

        return 0;

}
