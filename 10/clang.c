#include <stdbool.h>
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
    fprintf(stderr, "ERROR Unknown char %c\n", begin);
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
    fprintf(stderr, "ERROR Invalid char: %c\n", end);
    return -1;
  }
}

int checkChunk(char openChar, char *start, char *end, char endChar) {

  printf("checking Chunk from %c Chunk:", openChar);
  for (char *p = start; p <= end; p++)
    putchar(*p);
  putchar('\n');

  if (getEnder(openChar) == -1) {
    printf("non opener Found(endchar %c)\n", endChar);
    return getScore(openChar);
  }

  if (end - start == 0) {
    return getScore(*end);
    int score = getScore(*end);
  }

  int depth = 0;
  bool foundMatch = false;
  char *p = start + 1;
  for (; p <= end; p++) {
    // printf("char %c\n", *p);
    if (*p == openChar)
      depth++;

    if (*p == getEnder(openChar)) {
      if (depth == 0) {
        printf("found match %ld\n", p - start);

        if (start + 1 != p) {
          printf("check inside chunk (endchar: %c)\n", endChar);
          int inner = checkChunk(*(start + 1), start + 1, p - 1, *p);
          if (inner > 0) {
            return inner;
          } else if (inner < 0) {
            printf("invalid closer %c\n", *p);
            return getScore(*p);
          }
        }

        if (end - p > 0) {
          // check rest of String
          printf("checking rest(endchar: %c)\n", endChar);
          int rest = checkChunk(*(p + 1), p + 1, end, endChar);
          if (rest > 0) {
            return rest;
          } else if (rest < 0 && endChar != 0) {
            return getScore(*p);
          }
        }

        // all okay
        return 0;
      } else
        depth--;
    }
  }

  // ignore unmachted openners
  if (!foundMatch) {
    if (endChar == 0) {
      printf("no matching closer found\n");
      return checkChunk(*(start + 1), start + 1, end, 0);
    } else {
      // inner Chunk invalid
      printf("inner chunk not complete\n");
      return getScore(endChar);
    }
  } else {
    // all okay
    return 0;
  }
}

int main(int argc, char **argv) {

  char line[1000];
  int totalScore = 0;
  while (fscanf(stdin, "%s", line) != EOF) {

    // find end of line
    char *end;
    for (int i = 0; line[i] != '\0'; i++) {
      end = &line[i];
      if (*end == '\n')
        break;
    }

    int score = checkChunk(line[0], &line[0], end, 0);
    totalScore = totalScore + score;

    printf("input: %s, score %d\n\n", line, score);
  }

  printf("Total Score: %d", totalScore);

  return 0;
}
