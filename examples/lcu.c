#include "../irelia.h"
#include <stdio.h>

int main(void) {

    LCUClient* client;
    if (lcu_new(&client)) {
        return -1;
    }

    RT* rt = new_rt();

    char* json;
    if (lcu_get(client, rt, "/lol-champ-select/v1/current-champion", &json)) {
        return -1;
    }

    printf("%s", json);

    drop_rt(rt);
    lcu_drop(&client);

    return 0;
}