#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

char ** splitline(char * line, const char * delim) {
    char ** tokens = malloc(sizeof(char*) * 64);
    char * tkn;

    //char delim[10] = " \t\n\r\a";

    int pos = 0, bufsize = 64;

    if (tokens == NULL) {
        fprintf(stderr, "\nRSH ERROR: buffer allocation failed");
        exit(EXIT_FAILURE);
    }

    tkn = strtok(line, delim);
    while (tkn != NULL) {
        tokens [pos] = tkn;
        pos++;

        if (pos >= bufsize) {
            line = realloc(line, bufsize * sizeof(char*));

            if (!line) {
                fprintf(stderr, "\nRSH ERROR: buffer reallocation failed");
                exit(EXIT_FAILURE);
            }
        }

        tkn = strtok(NULL, delim);
    }

    tokens [pos] = NULL;

    return tokens;
}


int rsh_exec(char * argv) {

    char ** args = splitline(argv, " \n\t\a\r");

    pid_t pid, wpid;
	int status;
	pid = fork();
	if (pid == 0) {
		if (execvp(args[0], args) == -1) {
			perror("Rust SHell: ");
		}
        exit(EXIT_FAILURE);
	} else if (pid < 0) {
		perror("Rust SHell: ");
	} else {
        do {
            wpid = waitpid(pid, &status, WUNTRACED);
        } while (!WIFEXITED(status) && !WIFSIGNALED(status));
    }

    free(args);

    return 1;
}

/*
void rsh_exec_cmd(char *** tkns, int cnt, int root) {
    if (cnt >= 0) {
        int pfd[2];

        pid_t pid;

        if (root) {
            if (pipe(pfd) < 0) {
                perror("RSH::pipe error: failed on creating a pipe");
                exit(EXIT_FAILURE);
            }
        }

        if ((pid = fork()) < 0) {
            perror("RSH::fork error: failed on forking process");
            exit(EXIT_FAILURE);
        }

        if (pid == 0) {
            rsh_exec_cmd(tkns, cnt - 1, 0);

            if (!root) {
                close(pfd[0]);
                dup2(pfd[1], 1);
                close(pfd[1]);
            } else {
                close(pfd[1]);
                dup2(pfd[0], 0);
                close(pfd[0]);
            }

            execvp(tkns[cnt][0], tkns[cnt]);
            exit(EXIT_FAILURE);
        } else {
            int status;

            do {
                waitpid(pid, &status, WUNTRACED);
            } while (!WIFEXITED(status) && !WIFSIGNALED(status));

            close(pfd[0]);
            close(pfd[1]);
        }
    }
}

int rsh_exec(char * argv) {
    char ** tkn_l = splitline(argv);

    int tkn_cnt = 0, ;
    char *** result = malloc(sizeof(**result) * tkn_cnt);
}
*/