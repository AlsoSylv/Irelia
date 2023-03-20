/// Event type you're requesting from the socket
enum class EventTypeC {
  JsonApiEvent,
  LcdEvent,
  JsonApiEventCallback,
  LcdEventCallback,
};

/// Custom errors for the LCU
enum class LcuResponse {
  /// Things went as expected
  Success = 0,
  /// Expected or input type are incorrect
  FailedParseJson = 10,
  /// The LCU stopped running
  LCUStoppedRunning = 11,
  /// The game stopped running
  LeagueStoppedRunning = 12,
  /// The following request is invalid
  InvalidRequest = 13,
  /// The request body is invalid
  InvalidBody = 14,
  /// The LCU was never running
  LCUProcessNotRunning = 15,
  /// Could not locate port for the LCU
  PortNotFound = 16,
  /// The sub process could not be spawned
  CannotLaunchTerminal = 17,
  /// Auth token for the LCU could not be found
  AuthTokenNotFound = 18,
};

/// Enum representation of different team IDs
enum class TeamID {
  ALL,
  UNKNOWN,
  ORDER,
  CHAOS,
  NEUTRAL,
};

/// Struct that handles connections to the in-game API
/// holding a refernce to the hyper client and url
struct InGameClient;

/// Struct with methods that handles connections to the LCU
struct LCUClient;

/// ```rs
/// async fn web_socket() {
///     use irelia::ws::LCUWebSocket;
///
///     let ws = LCUWebSocket::new().unwrap();
///     ws.subscribe("OnJsonApiEvent");
///     loop {
///         let data = ws.client_reciver.unwrap();
///     }
/// }
/// ```
struct LCUWebSocket;

struct RT;

/// Event to send to the socket, endpoint is ignored
/// if you send JsonApiEvent and LcdEvent, and cannot
/// be null otherwise
struct Event {
  EventTypeC event;
  const char *endpoint;
};

extern "C" {

LcuResponse new_in_game(InGameClient **client);

LcuResponse all_game_data(InGameClient *client, RT *rt, char **json);

LcuResponse active_player(InGameClient *client, RT *rt, char **json);

LcuResponse active_player_name(InGameClient *client, RT *rt, char **json);

LcuResponse active_player_abilities(InGameClient *client, RT *rt, char **json);

LcuResponse active_player_runes(InGameClient *client, RT *rt, char **json);

LcuResponse player_list(InGameClient *client, RT *rt, char **json, const TeamID *team);

LcuResponse player_scores(InGameClient *client, RT *rt, char **json, const char *summoner);

LcuResponse player_summoner_spells(InGameClient *client, RT *rt, char **json, const char *summoner);

LcuResponse player_main_runes(InGameClient *client, RT *rt, char **json, const char *summoner);

LcuResponse player_items(InGameClient *client, RT *rt, char **json, const char *summoner);

LcuResponse event_data(InGameClient *client, RT *rt, char **json, const int *event_id);

LcuResponse game_stats(InGameClient *client, RT *rt, char **json);

/// Drops in game handle
void in_game_drop(InGameClient **game);

/// Drops the game response
void in_game_drop_res(char **res);

/// Creates a new LCU handle
LcuResponse lcu_new(LCUClient **client);

/// Makes a get request to the LCU
LcuResponse lcu_get(LCUClient *client, RT *rt, char *endpoint, char **c_json);

/// Makes a post request to the LCU
/// takes a string as a body that
/// must be json, else it will panic
LcuResponse lcu_post(LCUClient *client, RT *rt, char *endpoint, char *body, char **c_json);

/// Makes a put request to the LCU
/// takes a string as a body that
/// must be json, else it will panic
LcuResponse lcu_put(LCUClient *client, RT *rt, char *endpoint, char *body, char **c_json);

/// Makes a delete request to the LCU
LcuResponse lcu_delete(LCUClient *client, RT *rt, char *endpoint, char **c_json);

/// Makes a head request to the LCU
LcuResponse lcu_head(LCUClient *client, RT *rt, char *endpoint, char **c_json);

/// Drops the client handle
void lcu_drop(LCUClient **client);

/// Drops the client response
void lcu_drop_res(char **res);

RT *new_rt();

void drop_rt(RT *rt);

/// Creates a new handle for the web socket
LcuResponse new_ws(LCUWebSocket **client, RT *rt);

/// Subscribes to a new web socket event
void subscribe(LCUWebSocket *client, Event event);

/// Unsubscribes from a web socket event
void unsubscribe(LCUWebSocket *client, Event event);

/// Requests to the event sent by the websocket, returns null
/// if there is no event or if there is an error
LcuResponse next(LCUWebSocket *client, RT *rt, char **json);

/// Drops the web socket handle
void drop_ws(LCUWebSocket **client);

/// Drops the web socket response
void drop_ws_res(char **res);

} // extern "C"
