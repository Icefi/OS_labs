#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int get_count(char** ptr)
{
    int count = 0;
    while (ptr[count] != NULL) {
        count++;
    }
    return count;
}
void rsh_free_tokens(char** tokens_list, int tokens_count)
{
    for (int i = 0; i < tokens_count; i++)
        free(tokens_list[i]);
    free(tokens_list);
}
char** rsh_get_tokens(char* str, int* tokens_count)
{
    int number_of_malloc_tokens = 10;
    char* token = malloc(sizeof(*token) * 255);
    char** tmp = malloc(sizeof(*tmp) * number_of_malloc_tokens);

    for (int i = 0, token_len = 0; i < strlen(str); i++) {

        if ((str[i] == '\n' || str[i] == '\t' || str[i] == ' ') && token_len > 0) {

            token[token_len] = '\0';

            if ((*tokens_count) == number_of_malloc_tokens) {
                number_of_malloc_tokens += 3;
                tmp = realloc(tmp, number_of_malloc_tokens * sizeof(*tmp));
            }

            tmp[(*tokens_count)] = malloc(sizeof(**tmp) * token_len);
            strcpy(tmp[(*tokens_count)], token);
            (*tokens_count) += 1, token_len = 0;

        } else if (str[i] != '\n' && str[i] != '\t' && str[i] != ' ') {
            token[token_len++] = str[i];
        }
    }

    free(token);

    return tmp;
}
char** rsh_next(char** tokens_list, int tokens_count, int start, int* count)
{
    char** tmp = malloc(sizeof(*tmp) * (tokens_count - start));
    for (int i = start, j = 0; i < tokens_count && strcmp(tokens_list[i], "|") != 0; i++, j++, (*count) += 1) {
        tmp[j] = tokens_list[i];
    }
    tmp[(*count)] = NULL;
    return tmp;
}

void rsh_exec_cmd(char*** tokens_list, int count)
{
    int status;
    pid_t pid, ppid;

    if (count > 1) {
        ppid = fork();
        if (ppid == 0) {
            int fd[count][2];
            for (int i = 0; i < count; i++)
                if (pipe(fd[i]) < 0)
                    perror("pipe"), exit(EXIT_FAILURE);

            for (int i = 0; i < count; i++) {

                pid = fork();
                if (pid == 0) {
                    dup2(fd[i][STDIN_FILENO], STDIN_FILENO);
                    close(fd[i][STDOUT_FILENO]);
                    if (i + 1 != count) {
                        dup2(fd[i + 1][STDOUT_FILENO], STDOUT_FILENO);
                        close(fd[i + 1][STDIN_FILENO]);
                    }
                    execvp(tokens_list[i][0], tokens_list[i]);
                    perror("execvp"), exit(EXIT_FAILURE);
                }
                close(fd[i + 1][STDOUT_FILENO]);
                do {
                    waitpid(pid, &status, WUNTRACED);
                } while (!WIFEXITED(status) && !WIFSIGNALED(status));
            }
            exit(EXIT_SUCCESS);
        } else {
            int sp;
            do {
                waitpid(ppid, &sp, WUNTRACED);
            } while (!WIFEXITED(sp) && !WIFSIGNALED(sp));
        }
    } else {
        pid = fork();

        if (pid == 0) {
            execvp(tokens_list[0][0], tokens_list[0]);
            fprintf(stderr, "RSH::execution error: failed at executing binary module");
            exit(EXIT_FAILURE);
        }

        do {
            waitpid(pid, &status, WUNTRACED);
        } while (!WIFEXITED(status) && !WIFSIGNALED(status));
    }
}


void rsh_exec(char * argv) {
    for (int i = 0;; i++) {
        if (argv[i] == '\0') {
            argv[i] = '\n';
            break;
        }
    }

    int tokens_count = 0, start = 0, count = 0, iterations = 0;

    char** tokens_list = rsh_get_tokens(argv, &tokens_count);

    if (tokens_list == NULL) {
        fprintf(stderr, "RSH::allocation error: failed at allocating memory");
        exit(EXIT_FAILURE);
    }

    char*** result = malloc(sizeof(**result) * tokens_count);

    if (result == NULL) {
        rsh_free_tokens(tokens_list, tokens_count);
        fprintf(stderr, "RSH::allocation error: failed at allocating memory");
        exit(EXIT_FAILURE);
    }

    do {
        result[iterations++] = rsh_next(tokens_list, tokens_count, start, &count);
        start += count + 1, count = 0;
    } while (start < tokens_count);

    rsh_exec_cmd(result, iterations);
    rsh_free_tokens(tokens_list, tokens_count);

    for (int i = 0; i < iterations; i++)
        free(result[i]);

    free(result);
}
