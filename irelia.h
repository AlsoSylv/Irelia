#if defined(WS)
/**
 * Event type you're requesting from the socket
 */
typedef enum EventTypeC {
#if defined(WS)
  JsonApiEvent,
#endif
#if defined(WS)
  LcdEvent,
#endif
#if defined(WS)
  JsonApiEventCallback,
#endif
#if defined(WS)
  LcdEventCallback,
#endif
} EventTypeC;
#endif

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
#if defined(IN_GAME)
  /**
   * The game stopped running
   */
  LeagueStoppedRunning = 12,
#endif
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

#if defined(IN_GAME)
/**
 * Enum representation of different team IDs
 */
typedef enum TeamID {
#if defined(IN_GAME)
  ALL,
#endif
#if defined(IN_GAME)
  UNKNOWN,
#endif
#if defined(IN_GAME)
  ORDER,
#endif
#if defined(IN_GAME)
  CHAOS,
#endif
#if defined(IN_GAME)
  NEUTRAL,
#endif
} TeamID;
#endif

#if defined(REST)
typedef struct Future Future;
#endif

#if defined(IN_GAME)
/**
 * Struct that handles connections to the in-game API
 * holding a refernce to the hyper client and url
 */
typedef struct InGameClient InGameClient;
#endif

#if defined(REST)
/**
 * Struct with methods that handles connections to the LCU
 */
typedef struct LCUClient LCUClient;
#endif

#if defined(WS)
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
#endif

typedef struct RT RT;

#if defined(WS)
/**
 * Event to send to the socket, endpoint is ignored
 * if you send JsonApiEvent and LcdEvent, and cannot
 * be null otherwise
 */
typedef struct Event {
  enum EventTypeC event;
  const char *endpoint;
} Event;
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if defined(IN_GAME)
enum LcuResponse new_in_game(struct InGameClient **client);
#endif

#if defined(IN_GAME)
struct Future *all_game_data(struct InGameClient *client,
                             struct RT *rt,
                             void (*func)(char*, enum LcuResponse));
#endif

#if defined(IN_GAME)
struct Future *active_player(struct InGameClient *client,
                             struct RT *rt,
                             void (*func)(char*, enum LcuResponse));
#endif

#if defined(IN_GAME)
struct Future *active_player_name(struct InGameClient *client,
                                  struct RT *rt,
                                  void (*func)(char*, enum LcuResponse));
#endif

#if defined(IN_GAME)
struct Future *active_player_abilities(struct InGameClient *client,
                                       struct RT *rt,
                                       void (*func)(char*, enum LcuResponse));
#endif

#if defined(IN_GAME)
struct Future *active_player_runes(struct InGameClient *client,
                                   struct RT *rt,
                                   void (*func)(char*, enum LcuResponse));
#endif

#if defined(IN_GAME)
struct Future *player_list(struct InGameClient *client,
                           struct RT *rt,
                           void (*func)(char*, enum LcuResponse),
                           const enum TeamID *team);
#endif

#if defined(IN_GAME)
struct Future *player_scores(struct InGameClient *client,
                             struct RT *rt,
                             void (*func)(char*, enum LcuResponse),
                             char *summoner);
#endif

#if defined(IN_GAME)
struct Future *player_summoner_spells(struct InGameClient *client,
                                      struct RT *rt,
                                      void (*func)(char*, enum LcuResponse),
                                      char *summoner);
#endif

#if defined(IN_GAME)
struct Future *player_main_runes(struct InGameClient *client,
                                 struct RT *rt,
                                 void (*func)(char*, enum LcuResponse),
                                 char *summoner);
#endif

#if defined(IN_GAME)
struct Future *player_items(struct InGameClient *client,
                            struct RT *rt,
                            void (*func)(char*, enum LcuResponse),
                            char *summoner);
#endif

#if defined(IN_GAME)
struct Future *event_data(struct InGameClient *client,
                          struct RT *rt,
                          void (*func)(char*, enum LcuResponse),
                          const int *event_id);
#endif

#if defined(IN_GAME)
struct Future *game_stats(struct InGameClient *client,
                          struct RT *rt,
                          void (*func)(char*, enum LcuResponse));
#endif

#if defined(IN_GAME)
/**
 * Drops in game handle
 */
void in_game_drop(struct InGameClient **game);
#endif

#if defined(IN_GAME)
/**
 * Drops the game response
 */
void in_game_drop_res(char **res);
#endif

#if defined(REST)
/**
 * SAFETY: The future cannot be null
 */
int is_finished(struct Future *future);
#endif

#if defined(REST)
/**
 * SAFETY: The future here cannot be null
 * Dropping the future will abort it
 */
void drop_future(struct Future **future);
#endif

#if defined(REST)
/**
 * SAFETY: This takes pointers to arrays, these arrays
 * must be the same size, and futures cannot be null.
 * This also takes a non-null tokio runtime used to
 * execture futures
 */
void await_future(struct Future *future, struct RT *rt);
#endif

#if defined(REST)
/**
 * Creates a new LCU handle
 */
enum LcuResponse lcu_new(struct LCUClient **client);
#endif

#if defined(REST)
/**
 * SAFETY: None of these params can be null
 * the string passed to the function can be
 * null if there is no response or if there
 * is an error
 */
struct Future *lcu_get(struct LCUClient *client,
                       struct RT *rt,
                       const char *endpoint,
                       void (*func)(char*, enum LcuResponse));
#endif

#if defined(REST)
/**
 * SAFETY: None of these params can be null
 * the string passed to the function can be
 * null if there is no response or if there
 * is an error
 */
struct Future *lcu_post(struct LCUClient *client,
                        struct RT *rt,
                        const char *endpoint,
                        char *body,
                        void (*func)(char*, enum LcuResponse));
#endif

#if defined(REST)
/**
 * SAFETY: None of these params can be null
 * the string passed to the function can be
 * null if there is no response or if there
 * is an error
 */
struct Future *lcu_put(struct LCUClient *client,
                       struct RT *rt,
                       const char *endpoint,
                       char *body,
                       void (*func)(char*, enum LcuResponse));
#endif

#if defined(REST)
/**
 * SAFETY: None of these params can be null
 * the string passed to the function can be
 * null if there is no response or if there
 * is an error
 */
struct Future *lcu_delete(struct LCUClient *client,
                          struct RT *rt,
                          const char *endpoint,
                          void (*func)(char*, enum LcuResponse));
#endif

#if defined(REST)
/**
 * SAFETY: None of these params can be null
 * the string passed to the function can be
 * null if there is no response or if there
 * is an error
 */
struct Future *lcu_head(struct LCUClient *client,
                        struct RT *rt,
                        const char *endpoint,
                        void (*func)(char*, enum LcuResponse));
#endif

#if defined(REST)
/**
 * Drops the client handle
 */
void lcu_drop(struct LCUClient **client);
#endif

#if defined(REST)
/**
 * Drops the client response
 */
void lcu_drop_res(char **res);
#endif

struct RT *new_rt(void);

void drop_rt(struct RT *rt);

#if defined(WS)
/**
 * Creates a new handle for the web socket
 */
enum LcuResponse new_ws(struct LCUWebSocket **client, struct RT *rt);
#endif

#if defined(WS)
/**
 * Subscribes to a new web socket event
 */
void subscribe(struct LCUWebSocket *client, struct Event event);
#endif

#if defined(WS)
/**
 * Unsubscribes from a web socket event
 */
void unsubscribe(struct LCUWebSocket *client, struct Event event);
#endif

#if defined(WS)
/**
 * Requests to the event sent by the websocket, returns null
 * if there is no event or if there is an error
 */
enum LcuResponse next(struct LCUWebSocket *client, struct RT *rt, char **json);
#endif

#if defined(WS)
/**
 * Drops the web socket handle
 */
void drop_ws(struct LCUWebSocket **client);
#endif

#if defined(WS)
/**
 * Drops the web socket response
 */
void drop_ws_res(char **res);
#endif

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
