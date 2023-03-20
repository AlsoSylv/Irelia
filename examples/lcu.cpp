#include "../irelia.hpp"

int main(void) {

    LCUClient* client;
    if ((int) lcu_new(&client)) {
        return -1;
    }

    RT* rt = new_rt();

    char* json;
    if ((int) lcu_get(client, rt, (char*)"/lol-champ-select/v1/current-champion", &json)) {
        return -1;
    }

    printf("%s", json);

    drop_rt(rt);
    lcu_drop(&client);

    return 0;
}