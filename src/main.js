const { invoke } = window.__TAURI__.tauri;

document.getElementById('metadata-form').addEventListener('submit', async (event) => {
    event.preventDefault();

    const folderPath = document.getElementById('folder-path').value;
    const startTimecode = document.getElementById('start-timecode').value || '00:00:00:00';
    const outputFile = document.getElementById('output-file').value;
    const messageBox = document.getElementById('message-box');
    messageBox.textContent = '';  // Clear previous messages

    try {
        await invoke('generate_metadata', { folderPath, startTimecode, outputFile });
    } catch (error) {
        document.getElementById('output').textContent = `Error: ${error}`;
    }
});

window.__TAURI__.event.listen('log-message', event => {
    const messageBox = document.getElementById('message-box');
    messageBox.textContent += `${event.payload}\n`;
    messageBox.scrollTop = messageBox.scrollHeight;  // Scroll to the bottom
});
