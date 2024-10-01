// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// Constant
enum { OPT_DESSELECT, OPT_ADD, OPT_SHOW, OPT_EXECUTE, OPT_EXIT, FULLSIZE };

// Variables

struct Process {
  char name[10];
  int entry_time, execution_time;
  struct Process *next;
};

struct Process *start = NULL;
struct Process *end = NULL;
struct Process *aux;
int reference = 0;

// Prototypes
int menu();
void execute_fifo_scaling();
void add_proccess();
void show();
void free_mem();
void sort_fifo();
void swap_pointers(struct Process *a, struct Process *b);

int main(int argc, char *argv[]) {
  printf("Processor Scaling using FIFO (First in First Out)\n");
  int option = OPT_DESSELECT;
  while (option != OPT_EXIT) {
    option = menu();
    switch (option) {
      case OPT_ADD:
        add_proccess();
        break;
      case OPT_SHOW:
        show();
        break;
      case OPT_EXECUTE:
        execute_fifo_scaling();
        break;
      case OPT_EXIT:
        free_mem();
        break;
      default:
        printf("Option doesn't exists\nTry again\n\n");
    }
  }
  return EXIT_SUCCESS;
}

int menu() {
  int option;
  printf("\nChoose one action\n");
  printf("%d - Add a Proccess\n", OPT_ADD);
  printf("%d - Show the Processes\n", OPT_SHOW);
  printf("%d - Execute the processes\n", OPT_EXECUTE);
  printf("%d - Exit\n\n", OPT_EXIT);
  scanf("%d", &option);
  return option;
}

void add_proccess() {
  struct Process *new_item = malloc(sizeof(struct Process));
  if (start == NULL) {
    start = new_item;
    end = new_item;
  } else {
    end->next = new_item;
    end = new_item;
  }
  printf("Insert the name of the Proccess: ");
  scanf("%s", new_item->name);
  printf("Now tell the Entry time of %s: ", new_item->name);
  scanf("%d", &new_item->entry_time);
  printf("Last, tell us the Execution Time: ");
  scanf("%d", &new_item->execution_time);
  printf("Proccess added succesfully\n");
  reference++;
}

void show() {
  if (start == NULL) {
    printf("No Proccess found\n");
  } else {
    printf("Consulting all the proccesses\n");
    aux = start;
    while (aux != NULL) {
      printf("\nProccess name: %s\n", aux->name);
      printf("Entry Time: %d\n", aux->entry_time);
      printf("Proccess Time: %d\n", aux->execution_time);
      aux = aux->next;
    }
  }
}

void free_mem() {
  aux = start;
  while (aux != NULL) {
    if (aux->next != NULL) {
      aux = aux->next;
      free(start);
      start = aux;
    } else {
      free(aux);
      break;
    }
    aux = aux->next;
  }
}

void execute_fifo_scaling() {
  if (start == NULL) {
    printf("Couldn't find any proccess\n\n");
  } else {
    sort_fifo();
    while (start != NULL) {
      printf("Executing...\n");
      printf("\nProccess name: %s\n", start->name);
      printf("Entry Time: %d\n", start->entry_time);
      printf("Proccess Time: %d\n", start->execution_time);
      sleep(start->execution_time);
      if (start->next != NULL) {
        aux = start->next;
        free(start);
        start = aux;
      } else {
        free(start);
        start = NULL;
        end = NULL;
      }
      reference--;
    }
  }
}

void sort_fifo() {
  struct Process *prox;
  if (start != NULL) {
    aux = start;
    while (aux != NULL) {
      prox = aux->next;
      if (prox != NULL) {
        if (aux->entry_time > prox->entry_time) {
          swap_pointers(aux, prox);
          aux = aux->next;
        } else {
          aux = aux->next;
        }
      } else {
        break;
      }
    }
  }
}

void swap_pointers(struct Process *a, struct Process *b) {
  struct Process tmp = *a;
  *a = *b;
  *b = tmp;
}
