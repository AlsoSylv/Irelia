#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Custom errors for the LCU
enum class Error {
  /// Expected or input type are incorrect
  FailedParseJson = 0,
  /// The LCU stopped running
  LCUStoppedRunning = 1,
  /// The game stopped running
  LeagueStoppedRunning = 2,
  /// The following request is invalid
  InvalidRequest = 3,
  /// The request body is invalid
  InvalidBody = 4,
  /// The LCU was never running
  LCUProcessNotRunning = 5,
  /// Could not locate port for the LCU
  PortNotFound = 6,
  /// The sub process could not be spawned
  CannotLaunchTerminal = 7,
  /// Auth token for the LCU could not be found
  AuthTokenNotFound = 8,
};

struct LCUC;

template<typename T = void, typename E = void>
struct Result;

extern "C" {

Result<LCUC, Error> *lcu_new();

const Result<const char*, Error> *lcu_get(LCUC *client, char *endpoint);

const Result<const char*, Error> *lcu_post(LCUC *client, char *endpoint, char *body);

const Result<const char*, Error> *lcu_put(LCUC *client, char *endpoint, char *body);

const Result<const char*, Error> *lcu_delete(LCUC *client, char *endpoint);

const Result<const char*, Error> *lcu_head(LCUC *client, char *endpoint);

void lcu_drop(LCUC *client);

} // extern "C"
