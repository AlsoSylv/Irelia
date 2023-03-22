#include "../irelia.h"
#include <stdio.h>
#include <stdbool.h>

void printer(char* string, LcuResponse num) {
    printf("%s", string);
    printf("%d", num);
}

int main(void) {
    RT* rt = new_rt();
    LCUClient* client;
    LcuResponse error = lcu_new(&client);
    if (!error) {
        return -1;
    }

    Future* future = lcu_get(client, rt, "/lol-champ-select/v1/current-champion", *printer);

    while (true)
        if (is_finished(future))
            break;
    
    await_future(future, rt);
    drop_future(&future);
}