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
    link: string,
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

export type GetStreamsResponse = {
    success: boolean,
    streams: any[],
    error: string,
}