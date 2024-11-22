// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

int main(int argc, char *argv[]) {
  char a[100] = "36578";
  if (!reference_parser(a)) {
    printf("Erro!");
  }
  return EXIT_SUCCESS;
}

int menu() {
  int op = 0;
  printf("Memory Pagination");
  printf("\nChoose an option:\n\n");
  printf("1. FIFO\t2. Optimal\n");
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
  printf("%d Inserted at the queue\n", new_item->value);
}

bool pop(int *number) {
  if (start == NULL) {
    printf("Empty Queue\n");
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
