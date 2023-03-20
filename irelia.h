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
 * Custom errors for the LCU
 */
typedef enum LcuResponse {
  /**
   * Things went as expected
   */
  Success = 0,
  /**
   * Expected or input type are incorrect
   */
  FailedParseJson = 10,
  /**
   * The LCU stopped running
   */
  LCUStoppedRunning = 11,
  /**
   * The game stopped running
   */
  LeagueStoppedRunning = 12,
  /**
   * The following request is invalid
   */
  InvalidRequest = 13,
  /**
   * The request body is invalid
   */
  InvalidBody = 14,
  /**
   * The LCU was never running
   */
  LCUProcessNotRunning = 15,
  /**
   * Could not locate port for the LCU
   */
  PortNotFound = 16,
  /**
   * The sub process could not be spawned
   */
  CannotLaunchTerminal = 17,
  /**
   * Auth token for the LCU could not be found
   */
  AuthTokenNotFound = 18,
} LcuResponse;

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
 * Struct that handles connections to the in-game API
 * holding a refernce to the hyper client and url
 */
typedef struct InGameClient InGameClient;

/**
 * Struct with methods that handles connections to the LCU
 */
typedef struct LCUClient LCUClient;

/**
 * ```rs
 * async fn web_socket() {
 *     use irelia::ws::LCUWebSocket;
 *
 *     let ws = LCUWebSocket::new().unwrap();
 *     ws.subscribe("OnJsonApiEvent");
 *     loop {
 *         let data = ws.client_reciver.unwrap();
 *     }
 * }
 * ```
 */
typedef struct LCUWebSocket LCUWebSocket;

typedef struct RT RT;

/**
 * Event to send to the socket, endpoint is ignored
 * if you send JsonApiEvent and LcdEvent, and cannot
 * be null otherwise
 */
typedef struct Event {
  enum EventTypeC event;
  const char *endpoint;
} Event;

enum LcuResponse new_in_game(struct InGameClient *client);

enum LcuResponse all_game_data(struct InGameClient *client, struct RT *rt, char **json);

enum LcuResponse active_player(struct InGameClient *client, struct RT *rt, char **json);

enum LcuResponse active_player_name(struct InGameClient *client, struct RT *rt, char **json);

enum LcuResponse active_player_abilities(struct InGameClient *client, struct RT *rt, char **json);

enum LcuResponse active_player_runes(struct InGameClient *client, struct RT *rt, char **json);

enum LcuResponse player_list(struct InGameClient *client,
                             struct RT *rt,
                             char **json,
                             const enum TeamID *team);

enum LcuResponse player_scores(struct InGameClient *client,
                               struct RT *rt,
                               char **json,
                               const char *summoner);

enum LcuResponse player_summoner_spells(struct InGameClient *client,
                                        struct RT *rt,
                                        char **json,
                                        const char *summoner);

enum LcuResponse player_main_runes(struct InGameClient *client,
                                   struct RT *rt,
                                   char **json,
                                   const char *summoner);

enum LcuResponse player_items(struct InGameClient *client,
                              struct RT *rt,
                              char **json,
                              const char *summoner);

enum LcuResponse event_data(struct InGameClient *client,
                            struct RT *rt,
                            char **json,
                            const int *event_id);

enum LcuResponse game_stats(struct InGameClient *client, struct RT *rt, char **json);

/**
 * Drops in game handle
 */
void in_game_drop(struct InGameClient *game);

/**
 * Drops the game response
 */
void in_game_drop_res(char **res);

/**
 * Creates a new LCU handle
 */
enum LcuResponse lcu_new(struct LCUClient **client);

/**
 * Makes a get request to the LCU
 */
enum LcuResponse lcu_get(struct LCUClient *client, struct RT *rt, char *endpoint, char **c_json);

/**
 * Makes a post request to the LCU
 * takes a string as a body that
 * must be json, else it will panic
 */
enum LcuResponse lcu_post(struct LCUClient *client,
                          struct RT *rt,
                          char *endpoint,
                          char *body,
                          char **c_json);

/**
 * Makes a put request to the LCU
 * takes a string as a body that
 * must be json, else it will panic
 */
enum LcuResponse lcu_put(struct LCUClient *client,
                         struct RT *rt,
                         char *endpoint,
                         char *body,
                         char **c_json);

/**
 * Makes a delete request to the LCU
 */
enum LcuResponse lcu_delete(struct LCUClient *client, struct RT *rt, char *endpoint, char **c_json);

/**
 * Makes a head request to the LCU
 */
enum LcuResponse lcu_head(struct LCUClient *client, struct RT *rt, char *endpoint, char **c_json);

/**
 * Drops the client handle
 */
void lcu_drop(struct LCUClient **client);

/**
 * Drops the client response
 */
void lcu_drop_res(char **res);

struct RT *new_rt(void);

void drop_rt(struct RT *rt);

/**
 * Creates a new handle for the web socket
 */
enum LcuResponse new_ws(struct LCUWebSocket **client, struct RT *rt);

/**
 * Subscribes to a new web socket event
 */
void subscribe(struct LCUWebSocket *client, struct Event event);

/**
 * Unsubscribes from a web socket event
 */
void unsubscribe(struct LCUWebSocket *client, struct Event event);

/**
 * Requests to the event sent by the websocket, returns null
 * if there is no event or if there is an error
 */
enum LcuResponse next(struct LCUWebSocket *client, struct RT *rt, char **json);

/**
 * Drops the web socket handle
 */
void drop_ws(struct LCUWebSocket **client);

/**
 * Drops the web socket response
 */
void drop_ws_res(char **res);
