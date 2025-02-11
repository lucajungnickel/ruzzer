all:
	gcc -Wall crashy.c -o crashy
	gcc -Wall never_crashy.c -o never_crashy
	gcc -Wall CGI_crashy.c -o CGI_crashy -g
	gcc -Wall CGI_crashy.c -o CGI_crashy_asan -fsanitize=address -g