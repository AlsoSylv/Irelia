#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Event type you're requesting from the socket
enum class EventTypeC {
  JsonApiEvent,
  LcdEvent,
  JsonApiEventCallback,
  LcdEventCallback,
};

/// Enum representation of different team IDs
enum class TeamID {
  ALL,
  UNKNOWN,
  ORDER,
  CHAOS,
  NEUTRAL,
};

/// Opaque type for storing the Rust client
struct InGame;

/// Opaque pointer to the client and tokio runtime
struct Lcu;

/// Opaque type that stores the client
/// and the tokio runtime
struct LcuWS;

/// Stores a handle to the in game API, if
/// error != 0 then this handle is invalid
/// and cannot be used
struct NewInGame {
  InGame *client;
  int error;
};

/// Stores a response from the in game API
/// JSON should only be null if it errors
struct InGameResponse {
  char *json;
  int error;
};

/// Creates a handle to a client connection
/// if error != 0, then it did not connection
/// and client is  invalid
struct NewLCU {
  Lcu *client;
  int error;
};

/// Reponse from LCU endpoint, json can be null without
/// error, because some endpoints do not respond
struct LcuResponse {
  char *json;
  int error;
};

/// Handle to the LCU websocket
struct NewWS {
  LcuWS *client;
  int error;
};

/// Event to send to the socket, endpoint is ignored
/// if you send JsonApiEvent and LcdEvent, and cannot
/// be null otherwise
struct Event {
  EventTypeC event;
  const char *endpoint;
};

/// Holds JSON from the response, error can
/// be 0 and json can be null if no event
/// has been sent back
struct LcuWsRes {
  char *json;
  int error;
};

extern "C" {

NewInGame new_in_game();

InGameResponse all_game_data(InGame *client);

InGameResponse active_player(InGame *client);

InGameResponse active_player_name(InGame *client);

InGameResponse active_player_abilities(InGame *client);

InGameResponse active_player_runes(InGame *client);

InGameResponse player_list(InGame *client, const TeamID *team);

InGameResponse player_scores(InGame *client, const char *summoner);

InGameResponse player_summoner_spells(InGame *client, const char *summoner);

InGameResponse player_main_runes(InGame *client, const char *summoner);

InGameResponse player_items(InGame *client, const char *summoner);

InGameResponse event_data(InGame *client, const int *event_id);

InGameResponse game_stats(InGame *client);

/// Drops in game handle
void in_game_drop(NewInGame game);

/// Drops the game response
void in_game_drop_res(InGameResponse res);

/// Creates a new LCU handle
NewLCU lcu_new();

/// Makes a get request to the LCU
LcuResponse lcu_get(Lcu *client, const char *endpoint);

/// Makes a post request to the LCU
/// takes a string as a body that
/// must be json, else it will panic
LcuResponse lcu_post(Lcu *client, const char *endpoint, const char *body);

/// Makes a put request to the LCU
/// takes a string as a body that
/// must be json, else it will panic
LcuResponse lcu_put(Lcu *client, const char *endpoint, const char *body);

/// Makes a delete request to the LCU
LcuResponse lcu_delete(Lcu *client, const char *endpoint);

/// Makes a head request to the LCU
LcuResponse lcu_head(Lcu *client, const char *endpoint);

/// Drops the client handle
void lcu_drop(NewLCU client);

/// Drops the client response
void lcu_drop_res(LcuResponse res);

/// Creates a new handle for the web socket
NewWS new_ws();

/// Subscribes to a new web socket event
void subscribe(LcuWS *client, Event event);

/// Unsubscribes from a web socket event
void unsubscribe(LcuWS *client, Event event);

/// Requests to the event sent by the websocket, returns null
/// if there is no event or if there is an error
LcuWsRes next(LcuWS *client);

/// Drops the web socket handle
void drop_ws(NewWS client);

/// Drops the web socket response
void drop_ws_res(LcuWsRes res);

} // extern "C"
