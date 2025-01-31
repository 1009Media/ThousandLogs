const { invoke } = window.__TAURI__.core;

// Check if FFprobe is in the PATH
async function checkFFprobe() {
        try {
                const isFFprobeAvailable = await invoke('check_ffprobe');
                if (!isFFprobeAvailable) {
                        document.getElementById('warning-message').style.display = 'block';
                        document.getElementById('master-input-group').style.display = 'none';
                } else {
                        document.getElementById('warning-message').style.display = 'none';
                        document.getElementById('master-input-group').style.display = 'block';
                }
        } catch (error) {
                console.error('Error checking ffprobe:', error);
                document.getElementById('warning-message').style.display = 'block';
                document.getElementById('master-input-group').style.display = 'none';
        }
}

document.addEventListener('DOMContentLoaded', checkFFprobe);

//document.getElementById('master-input-group').addEventListener('submit', async (event) => {
//        event.preventDefault();
//
//        const rushesImportPathInput = document.getElementById('rushes-import-path-input').value;
//        const startTimecodeInput = document.getElementById('
