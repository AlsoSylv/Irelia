#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum EventTypeC {
  OnJsonApiEvent,
  OnLcdEvent,
  OnJsonApiEventCallback,
  OnLcdEventCallback,
} EventTypeC;

/**
 * Enum representation of different team IDs
 */
typedef enum TeamID {
  ALL,
  UNKNOWN,
  ORDER,
  CHAOS,
  NEUTRAL,
} TeamID;

typedef struct InGame InGame;

typedef struct Lcu Lcu;

typedef struct LcuWS LcuWS;

typedef struct NewInGame {
  struct InGame *client;
  int error;
} NewInGame;

typedef struct InGameResponse {
  char *json;
  int error;
} InGameResponse;

typedef struct NewLCU {
  struct Lcu *client;
  int error;
} NewLCU;

typedef struct LcuResponse {
  char *json;
  int error;
} LcuResponse;

typedef struct NewWS {
  struct LcuWS *client;
  int error;
} NewWS;

typedef struct Event {
  enum EventTypeC event;
  const char *endpoint;
} Event;

typedef struct LcuWsRes {
  char *json;
  int error;
} LcuWsRes;

struct NewInGame new_in_game(void);

struct InGameResponse all_game_data(struct InGame *client);

struct InGameResponse active_player(struct InGame *client);

struct InGameResponse active_player_name(struct InGame *client);

struct InGameResponse active_player_abilities(struct InGame *client);

struct InGameResponse active_player_runes(struct InGame *client);

struct InGameResponse player_list(struct InGame *client, const enum TeamID *team);

struct InGameResponse player_scores(struct InGame *client, const char *summoner);

struct InGameResponse player_summoner_spells(struct InGame *client, const char *summoner);

struct InGameResponse player_main_runes(struct InGame *client, const char *summoner);

struct InGameResponse player_items(struct InGame *client, const char *summoner);

struct InGameResponse event_data(struct InGame *client, const int *event_id);

struct InGameResponse game_stats(struct InGame *client);

void in_game_drop(struct InGame *game);

void in_game_drop_res(struct InGameResponse res);

struct NewLCU lcu_new(void);

struct LcuResponse lcu_get(struct Lcu *client, const char *endpoint);

struct LcuResponse lcu_post(struct Lcu *client, const char *endpoint, const char *body);

struct LcuResponse lcu_put(struct Lcu *client, const char *endpoint, const char *body);

struct LcuResponse lcu_delete(struct Lcu *client, const char *endpoint);

struct LcuResponse lcu_head(struct Lcu *client, const char *endpoint);

void lcu_drop(struct Lcu *client);

void lcu_drop_res(struct LcuResponse res);

struct NewWS new_ws(void);

void subscribe(struct LcuWS *client, struct Event event);

void unsubscribe(struct LcuWS *client, struct Event event);

struct LcuWsRes next(struct LcuWS *client);

void drop_ws(struct LcuWS *client);

void drop_ws_res(struct LcuWsRes res);
