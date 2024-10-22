#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Call: crashy <argument>\n");
        return 0;
    }
    printf("Hello\n");
    return -1;
    int len = strlen(argv[1]);
    if (len > 10) {
        printf("Input is too long!\n");
        return -1;
    }
}