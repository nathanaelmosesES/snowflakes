const XTERM_STATE_PREFIX = "xterm_state_";

export function saveTerminalState(key: string, state: string) {
    if (!key || key === "unknown") return;
    console.log("save terminal state", key, state);
    try {
        sessionStorage.setItem(XTERM_STATE_PREFIX + key, state);
        console.log(`[SessionState] Saved state for ${key}`);
    } catch (e) {
        console.error(`[SessionState] Failed to save state for ${key}:`, e);
    }
}

export function loadTerminalState(key: string): string | null {
    if (!key || key === "unknown") return null;
    // console.log("load terminal state", key);
    try {
        return sessionStorage.getItem(XTERM_STATE_PREFIX + key);
    } catch (e) {
        console.error(`[SessionState] Failed to load state for ${key}:`, e);
        return null;
    }
}

export function clearTerminalState(key: string) {
    if (!key || key === "unknown") return;
    try {
        sessionStorage.removeItem(XTERM_STATE_PREFIX + key);
    } catch (e) {
        console.error(`[SessionState] Failed to clear state for ${key}:`, e);
    }
}
