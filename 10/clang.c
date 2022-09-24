#include <stdio.h>

char getEnder(char begin) {
  switch (begin) {
  case '(':
    return ')';
  case '[':
    return ']';
  case '{':
    return '}';
  case '<':
    return '>';
  default:
    fprintf(stderr, "Unknown char %c", begin);
    return -1;
  }
}

int getScore(char end) {
  switch (end) {
  case ')':
    return 3;
  case ']':
    return 57;
  case '}':
    return 1197;
  case '>':
    return 25137;
  default:
    fprintf(stderr, "Invalid char: %c", end);
    return -1;
  }
}

int checkChunk(char openChar, char *start, char *end) {

  if (end - start == 1) {
    printf("@hook@\n");
    return 0;
  }

  int depth = 0;

  for (char *p = start + 1; p <= end; p++) {
    printf("char %c\n", *p);
    if (*p == openChar)
      depth++;

    if (*p == getEnder(openChar)) {
      if (depth == 0) {
        printf("found match %ld", p - start);

        // all okay
        return 0;
      } else
        depth--;
    }
  }

  // all okay
  return 0;
}

int main(int argc, char **argv) {

  char line[1000];
  fscanf(stdin, "%s", line);

  // find end of line
  char *end;
  for (int i = 0; line[i] != '\0'; i++) {
    end = &line[i];
    if (*end == '\n')
      break;
  }

  int score = checkChunk(line[0], &line[0], end);

  printf("input: %s, score %d", line, score);

  return 0;
}
