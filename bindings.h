#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Event type you're requesting from the socket
 */
typedef enum EventTypeC {
  JsonApiEvent,
  LcdEvent,
  JsonApiEventCallback,
  LcdEventCallback,
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

/**
 * Opaque type for storing the Rust client
 */
typedef struct InGame InGame;

/**
 * Opaque pointer to the client and tokio runtime
 */
typedef struct Lcu Lcu;

/**
 * Opaque type that stores the client
 * and the tokio runtime
 */
typedef struct LcuWS LcuWS;

/**
 * Stores a handle to the in game API, if
 * error != 0 then this handle is invalid
 * and cannot be used
 */
typedef struct NewInGame {
  struct InGame *client;
  int error;
} NewInGame;

/**
 * Stores a response from the in game API
 * JSON should only be null if it errors
 */
typedef struct InGameResponse {
  char *json;
  int error;
} InGameResponse;

/**
 * Creates a handle to a client connection
 * if error != 0, then it did not connection
 * and client is  invalid
 */
typedef struct NewLCU {
  struct Lcu *client;
  int error;
} NewLCU;

/**
 * Reponse from LCU endpoint, json can be null without
 * error, because some endpoints do not respond
 */
typedef struct LcuResponse {
  char *json;
  int error;
} LcuResponse;

/**
 * Handle to the LCU websocket
 */
typedef struct NewWS {
  struct LcuWS *client;
  int error;
} NewWS;

/**
 * Event to send to the socket, endpoint is ignored
 * if you send JsonApiEvent and LcdEvent, and cannot
 * be null otherwise
 */
typedef struct Event {
  enum EventTypeC event;
  const char *endpoint;
} Event;

/**
 * Holds JSON from the response, error can
 * be 0 and json can be null if no event
 * has been sent back
 */
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

/**
 * Drops in game handle
 */
void in_game_drop(struct NewInGame game);

/**
 * Drops the game response
 */
void in_game_drop_res(struct InGameResponse res);

/**
 * Creates a new LCU handle
 */
struct NewLCU lcu_new(void);

/**
 * Makes a get request to the LCU
 */
struct LcuResponse lcu_get(struct Lcu *client, const char *endpoint);

/**
 * Makes a post request to the LCU
 * takes a string as a body that
 * must be json, else it will panic
 */
struct LcuResponse lcu_post(struct Lcu *client, const char *endpoint, const char *body);

/**
 * Makes a put request to the LCU
 * takes a string as a body that
 * must be json, else it will panic
 */
struct LcuResponse lcu_put(struct Lcu *client, const char *endpoint, const char *body);

/**
 * Makes a delete request to the LCU
 */
struct LcuResponse lcu_delete(struct Lcu *client, const char *endpoint);

/**
 * Makes a head request to the LCU
 */
struct LcuResponse lcu_head(struct Lcu *client, const char *endpoint);

/**
 * Drops the client handle
 */
void lcu_drop(struct NewLCU client);

/**
 * Drops the client response
 */
void lcu_drop_res(struct LcuResponse res);

/**
 * Creates a new handle for the web socket
 */
struct NewWS new_ws(void);

/**
 * Subscribes to a new web socket event
 */
void subscribe(struct LcuWS *client, struct Event event);

/**
 * Unsubscribes from a web socket event
 */
void unsubscribe(struct LcuWS *client, struct Event event);

/**
 * Requests to the event sent by the websocket, returns null
 * if there is no event or if there is an error
 */
struct LcuWsRes next(struct LcuWS *client);

/**
 * Drops the web socket handle
 */
void drop_ws(struct NewWS client);

/**
 * Drops the web socket response
 */
void drop_ws_res(struct LcuWsRes res);
