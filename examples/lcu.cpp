#include "../irelia.h" 
#include "iostream"

void printer(char* string, LcuResponse num) {
    std::cout << string;
    std::cout << num;
}

int main(void) {

    LCUClient* client;
    if ((int) lcu_new(&client)) {
        return -1;
    }

    RT* rt = new_rt();

    Future* future = lcu_get(client, rt, (char*)"/lol-champ-select/v1/current-champion", *printer);

    while (true) 
        if (is_finished(future))
            break;

    drop_rt(rt);
    drop_future(&future);
    lcu_drop(&client);

    return 0;
}