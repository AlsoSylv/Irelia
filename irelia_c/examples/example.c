#include "../bindings.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    const struct RT* rt = new_rt();
    const struct RequestClient* rc = new_request_client();

    struct LCUClient* lc;
    const struct LCUResponse* res = new_lcu_client(rc, &lc);

    const char code = get_response_code(res);

    printf("%d \n", (int) code);

    if (code != 0) {
        printf("%s \n", get_response_description(res));

        free((void*) rt);
        free((void*) rc);
        free((void*) lc);
        free((void*) res);

        return 1;
    } else {
        Future* fut = lcu_get(lc, rt, "/lol-summoner/v1/current-summoner");

        char* res;
        const LCUResponse* get_res = block_on(fut, rt, &res);

        const char code = get_response_code(get_res);

        if (code != 0) {
            printf("%s \n", get_response_description(get_res));

            free((void*) rt);
            free((void*) rc);
            free((void*) lc);
            free((void*) get_res);
            free((void*) fut);

            return 1;
        } else {
            printf("%s \n", res);

            free((void*) rt);
            free((void*) rc);
            free((void*) lc);
            free((void*) get_res);
            free((void*) fut);
        }
    }

   return 0;
}