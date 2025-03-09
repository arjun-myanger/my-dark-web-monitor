const { invoke } = window.__TAURI__.tauri;

async function checkBreach(email) {
    try {
        const result = await invoke("check_breach", { email });
        return result;
    } catch (error) {
        console.error("Error fetching breaches:", error);
        return [];
    }
}
