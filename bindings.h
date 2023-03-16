#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct LCU;

struct NewLCU {
  LCU *client;
  int error;
};

struct LcuResponse {
  const char *json;
  int error;
};

extern "C" {

NewLCU lcu_new();

LcuResponse lcu_get(LCU *client, const char *endpoint);

LcuResponse lcu_post(LCU *client, const char *endpoint, const char *body);

LcuResponse lcu_put(LCU *client, const char *endpoint, const char *body);

LcuResponse lcu_delete(LCU *client, const char *endpoint);

LcuResponse lcu_head(LCU *client, const char *endpoint);

void lcu_drop(LCU *client);

} // extern "C"
