import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';
import type { SearchResponse, StreamLinkResponse, LoginResponse, CredsResponse, SetCredsResponse } from "./scc_def.ts";

// Create a store for the user's data
export let webshareToken = writable("");

// Function to handle user login
export async function login(username: string, password: string): Promise<boolean> {
    const response: LoginResponse = await invoke('cmd_login', { username: username, password: password });

    if (response.success) {
        webshareToken.set(response.token);
        return true;
    } else {
        console.error(response.error);
        return false;
    }
}

// Function to handle video search
export async function search(query: string): Promise<any> {
    const response: SearchResponse = await invoke('cmd_search', { query: query });
    return response;
}

// Function to handle video download/play
export async function getStreamLink(name: string, ident: string, token: string): Promise<string> {
    return new Promise(async (resolve, reject) => {
        const response: StreamLinkResponse = await invoke('cmd_get_stream_link', { name: name, ident: ident, token: token });
        if (response.success) {
            resolve(response.url);
        } else {
            console.error(response.error);
            reject(response.error);
        }
    });
}

export async function getCredentials(): Promise<[string, string]> {
    return new Promise(async (resolve, reject) => {
        const response: CredsResponse = await invoke('cmd_get_creds', {});
        if (response.success) {
            resolve(response.creds);
        } else {
            console.error(response.error);
            reject(response.error);
        }
    });
}

export async function saveCredentials(username: string, password: string): Promise<boolean> {
    return new Promise(async (resolve, reject) => {
        const response: SetCredsResponse = await invoke('cmd_save_creds', { username: username, password: password });
        if (response.success) {
            resolve(true);
        } else {
            console.error(response.error);
            reject(response.error);
        }
    });
}