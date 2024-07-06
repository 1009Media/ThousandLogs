const { invoke } = window.__TAURI__.tauri;

async function checkFFprobe() {
    try {
        const isFFprobeAvailable = await invoke('check_ffprobe');
        if (!isFFprobeAvailable) {
            document.getElementById('warning-message').style.display = 'block';
            document.getElementById('metadata-form').style.display = 'none';
        } else {
            document.getElementById('warning-message').style.display = 'none';
            document.getElementById('metadata-form').style.display = 'flex';
        }
    } catch (error) {
        console.error('Error checking ffprobe:', error);
        document.getElementById('warning-message').style.display = 'block';
        document.getElementById('metadata-form').style.display = 'none';
    }
}

document.addEventListener('DOMContentLoaded', checkFFprobe);

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

document.getElementById('browse-button').addEventListener('click', async () => {
    const selected = await window.__TAURI__.dialog.open({
        directory: true,
        multiple: false
    });
    if (selected) {
        document.getElementById('folder-path').value = selected;
    }
});

window.__TAURI__.event.listen('log-message', event => {
    const messageBox = document.getElementById('message-box');
    messageBox.textContent += `${event.payload}\n`;
    messageBox.scrollTop = messageBox.scrollHeight;  // Scroll to the bottom
});
