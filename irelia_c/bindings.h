typedef struct Future Future;

typedef struct LCUClient LCUClient;

typedef struct LCUResponse LCUResponse;

typedef struct RequestClient RequestClient;

typedef struct RT RT;

struct RT *new_rt(void);

void drop_rt(struct RT *rt);

struct LCUResponse *block_on(struct Future *fut, const struct RT *rt, char **res);

char is_finished(struct Future *fut);

struct RequestClient *new_request_client(void);

void drop_request_client(struct RequestClient *client);

struct LCUResponse *new_lcu_client(const struct RequestClient *client,
                                   struct LCUClient **lcu_client);

struct Future *lcu_delete(const struct LCUClient *client,
                          const struct RT *rt,
                          const char *endpoint);

struct Future *lcu_get(const struct LCUClient *client, const struct RT *rt, const char *endpoint);

struct Future *lcu_head(const struct LCUClient *client, const struct RT *rt, const char *endpoint);

struct Future *lcu_post(const struct LCUClient *client,
                        const struct RT *rt,
                        const char *endpoint,
                        const char *body);

struct Future *lcu_patch(const struct LCUClient *client,
                         const struct RT *rt,
                         const char *endpoint,
                         const char *body);

struct Future *lcu_put(const struct LCUClient *client,
                       const struct RT *rt,
                       const char *endpoint,
                       const char *body);

void drop_lcu_client(struct LCUClient *lcu_client);

char get_response_code(struct LCUResponse *res);

const char *get_response_description(struct LCUResponse *res);

void drop_future(struct Future *fut);

void drop_lcu_res(struct LCUResponse *res);
