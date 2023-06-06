typedef struct Future Future;

typedef struct LCUClient LCUClient;

typedef struct LCUResponse LCUResponse;

typedef struct RT RT;

typedef struct RequestClient RequestClient;

const struct RT *new_rt(void);

const struct LCUResponse *block_on(struct Future *fut, const struct RT *rt, char **res);

char is_finished(struct Future *fut);

const struct RequestClient *new_request_client(void);

const struct LCUResponse *new_lcu_client(const struct RequestClient *client,
                                         struct LCUClient **lcu_client);

struct Future *lcu_get(const struct LCUClient *client, const struct RT *rt, const char *endpoint);

struct Future *lcu_post(const struct LCUClient *client,
                        const struct RT *rt,
                        const char *endpoint,
                        const char *body);

char get_response_code(const struct LCUResponse *res);

const char *get_response_description(const struct LCUResponse *res);
