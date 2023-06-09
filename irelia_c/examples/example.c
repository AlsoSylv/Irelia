#include "../bindings.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    int return_val = 0;

    struct RT* rt = new_rt();
    struct RequestClient* rc = new_request_client();

    struct LCUClient* lc;
    struct LCUResponse* res = new_lcu_client(rc, &lc);

    const char code = get_response_code(res);

    printf("%d \n", (int) code);

    if (code != 0) {
        printf("%s \n", get_response_description(res));
        return_val = 1;
    } else {
        Future* fut = lcu_get(lc, rt, "/lol-summoner/v1/current-summoner");

        char* res;
        struct LCUResponse* get_res = block_on(fut, rt, &res);

        drop_future(fut);

        const char code = get_response_code(get_res);

        if (code != 0) {
            printf("%s \n", get_response_description(get_res));
            return_val = 1;
        } else 
            printf("%s \n", res);
        

        drop_lcu_res(get_res);
    }

    if (lc != NULL) 
        drop_lcu_client(lc);

    drop_lcu_res(res);
    drop_request_client(rc);
    drop_rt(rt);

    return return_val;
}