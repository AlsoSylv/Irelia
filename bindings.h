#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Lcu Lcu;

typedef struct NewLCU {
  struct Lcu *client;
  int error;
} NewLCU;

typedef struct LcuResponse {
  char *json;
  int error;
} LcuResponse;

struct NewLCU lcu_new(void);

struct LcuResponse lcu_get(struct Lcu *client, const char *endpoint);

struct LcuResponse lcu_post(struct Lcu *client, const char *endpoint, const char *body);

struct LcuResponse lcu_put(struct Lcu *client, const char *endpoint, const char *body);

struct LcuResponse lcu_delete(struct Lcu *client, const char *endpoint);

struct LcuResponse lcu_head(struct Lcu *client, const char *endpoint);

void lcu_drop(struct Lcu *client);

void lcu_drop_res(struct LcuResponse res);
