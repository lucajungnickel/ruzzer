#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>

char *decode_url(const char *src) {
    size_t len = strlen(src);
    char *decoded = malloc(len + 1); // Allocate memory for the decoded string
    if (!decoded) {
        return NULL; // Memory allocation failed
    }

    char *dst = decoded; // Pointer for the destination string
    for (size_t i = 0; i < len; ++i) {
        if (src[i] == '%') {
            // Convert hex code to character
            int value;
            sscanf(src + i + 1, "%2x", &value); // Read the hex value
            decoded[dst - decoded + 1] = (char)value;
            dst += 2;
            i += 2; // Skip the next two characters
        
        } else if (src[i] == '+') {
            *dst++ = ' '; // Convert '+' to space
        } else {
            *dst++ = src[i]; // Copy regular characters
        }
    }
    *dst = '\0'; // Null-terminate the decoded string

    return decoded;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Call: CGI_crashy <argument>\n");
        return 0;
    }
    printf("Try to decode %s...\n", argv[1]);
    //CGI decoder
    //Blanks are replaced with '+'
    //invalid characters are replaced with '%xx'
    char* decoded = decode_url(argv[1]);
    printf("Decoded:\n");
    printf("%s", decoded);

    free(decoded);
}