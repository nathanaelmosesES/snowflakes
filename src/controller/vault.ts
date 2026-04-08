import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';
import { appDataDir } from '@tauri-apps/api/path';

const initStronghold = async () => {
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const vaultPassword = 'vault password';
    const stronghold = await Stronghold.load(vaultPath, vaultPassword);

    let client: Client;
    const clientName = 'SnowflakesClient';
    try {
        client = await stronghold.loadClient(clientName);
    } catch {
        client = await stronghold.createClient(clientName);
    }

    return {
        stronghold,
        client,
    };
};

// Insert a record to the store
export async function insertRecord(store: any, key: string, value: string) {
    const data = Array.from(new TextEncoder().encode(value));
    await store.insert(key, data);
}

// Read a record from store
export async function getRecord(store: any, key: string): Promise<string> {
    const data = await store.get(key);
    return new TextDecoder().decode(new Uint8Array(data));
}

export const { stronghold, client } = await initStronghold();

