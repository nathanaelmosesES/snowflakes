import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';
import { appDataDir } from '@tauri-apps/api/path';
import type { SnowflakesSettings, SessionInfo, DefaultAccount } from '../types/settings';
import { DEFAULT_SETTINGS } from '../types/settings';

let _stronghold: Stronghold | null = null;
let _client: Client | null = null;

async function getClient(): Promise<Client> {
    if (_client) return _client;
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const vaultPassword = 'vault password'; // In production, this should be derived or entered by user
    _stronghold = await Stronghold.load(vaultPath, vaultPassword);

    const clientName = 'SnowflakesClient';
    try {
        _client = await _stronghold.loadClient(clientName);
    } catch {
        _client = await _stronghold.createClient(clientName);
    }
    return _client;
}

async function save(): Promise<void> {
    if (_stronghold) await _stronghold.save();
}

async function getStore() {
    const client = await getClient();
    return client.getStore();
}

export async function insertRecord(key: string, value: string): Promise<void> {
    const store = await getStore();
    const data = Array.from(new TextEncoder().encode(value));
    await store.insert(key, data);
    await save();
}

export async function getRecord(key: string): Promise<string> {
    const store = await getStore();
    const data = await store.get(key);
    if (!data || data.length === 0) return "";
    return new TextDecoder().decode(new Uint8Array(data));
}

export async function removeRecord(key: string): Promise<void> {
    const store = await getStore();
    try {
        await store.remove(key);
        await save();
    } catch {
        // ignore
    }
}

const ACCOUNT_KEY = "default_account";

export async function saveDefaultAccount(account: DefaultAccount): Promise<void> {
    await insertRecord(ACCOUNT_KEY, JSON.stringify(account));
}

export async function loadDefaultAccount(): Promise<DefaultAccount> {
    const raw = await getRecord(ACCOUNT_KEY);
    if (!raw) return { username: "", password: "" };
    try {
        return JSON.parse(raw);
    } catch {
        return { username: "", password: "" };
    }
}


export async function saveSessionPass(session_key: string, pass: string): Promise<void> {
    await insertRecord(session_key, pass);
}


export async function loadSessionPass(sessionKey: string): Promise<string | null> {
    const raw = await getRecord(sessionKey);
    if (!raw) return null;
    try {
        return raw;
    } catch {
        return null;
    }
}

// export async function loadAllSessions(): Promise<SessionInfo[]> {
//     const keys = await loadSessionKeys();

//     // Fetch all records in parallel
//     const sessionPromises = keys.map(async (key) => {
//         const raw = await getRecord(`session:${key}`);
//         if (raw) {
//             try {
//                 return JSON.parse(raw) as SessionInfo;
//             } catch {
//                 return null;
//             }
//         }
//         return null;
//     });

//     const results = await Promise.all(sessionPromises);
//     return results.filter((s): s is SessionInfo => s !== null);
// }

export async function deleteSessionPass(sessionKey: string): Promise<void> {
    await removeRecord(sessionKey);
}
