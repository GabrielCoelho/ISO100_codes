// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum {
  OPT_DESSELECT,
  OPT_FIFO,
  OPT_OPT,
  OPT_EXIT,
};

struct Mem {
  int value;
  struct Mem *next;
};

struct Mem *start = NULL;
struct Mem *end = NULL;
struct Mem *aux;
int reference = 0;

int menu();
void push(int number);
bool pop(int *number);
bool reference_parser(char *ref);
void fifo(int max);
void optimal(int max);
void terminator();

int main(int argc, char *argv[]) {
  system("clear");
  int option = OPT_DESSELECT;
  int max_page;
  char ref[30];
  while (option != OPT_EXIT) {
    option = menu();
    switch (option) {
      case OPT_FIFO:
        printf("Insert a reference: ");
        scanf("%s", ref);
        if (!reference_parser(ref)) {
          printf("Error while parsing!");
          return EXIT_FAILURE;
        }
        printf("Insert a page max number: ");
        scanf("%d", &max_page);
        fifo(max_page);
        break;
      case OPT_OPT:
        printf("Insert a reference: ");
        scanf("%s", ref);
        if (!reference_parser(ref)) {
          printf("Error while parsing!");
          return EXIT_FAILURE;
        }
        printf("Insert a page max number: ");
        scanf("%d", &max_page);
        optimal(max_page);
        break;
      case OPT_EXIT:
        printf("Exiting...");
        terminator();
        break;
    }
  }

  return EXIT_SUCCESS;
}

int menu() {
  int op = 0;
  printf("Memory Pagination\n\n");
  printf("%d. FIFO\t%d. Optimal\n%d. Exit", OPT_FIFO, OPT_OPT, OPT_EXIT);
  printf("\nChoose an option: ");
  scanf("%d", &op);
  return op;
}

void push(int number) {
  struct Mem *new_item = malloc(sizeof(struct Mem));
  new_item->value = number;
  new_item->next = NULL;
  if (start == NULL) {
    start = new_item;
    end = new_item;
  } else {
    end->next = new_item;
    end = new_item;
  }
  reference += 1;
  // printf("%d Inserted at the queue\n", new_item->value);
}

bool pop(int *number) {
  if (start == NULL) {
    /*printf("Empty Queue\n");*/
    return false;
  }
  aux = start;
  printf("Number %d removed from the queue\n", start->value);
  start = start->next;
  free(aux);
  return true;
}

bool reference_parser(char *ref) {
  for (int i = 0; i < strlen(ref); i++) {
    push(ref[i] - '0');
  }
  if (reference == strlen(ref)) {
    return true;
  }
  return false;
}

void fifo(int max) {
  int *memo = malloc(sizeof(int) * max);
  max = max - 1;
  int controller = 0;
  aux = start;
  for (int i = 0; i < reference; i++) {
    pop(&controller);
    printf("%d ", controller);
  }
  for (int i = 0; i < max; i++) {
    if (i == max && i < reference) {
      printf("\n");
      i = 0;
    }
    if (i >= reference) break;
  }

  free(memo);
}

void optimal(int max) {}

void terminator() {
  int not;
  if (reference > 0) {
    for (int i = 0; i < reference; i++) {
      pop(&not);
    }
  }
}
