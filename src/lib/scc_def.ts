export type SearchResponse = {
    _shards: {
        failed: number,
        successful: number,
        total: number
    },
    timed_out: boolean,
    took: number,
    hits: any,
};

export type StreamLinkResponse = {
    success: boolean,
    url: string,
    error: string,
};

export type LoginResponse = {
    success: boolean,
    token: string,
    error: string,
};

export type CredsResponse = {
    success: boolean,
    creds: [username: string, password: string],
    error: string,
}

export type SetCredsResponse = {
    success: boolean,
    error: string,
}