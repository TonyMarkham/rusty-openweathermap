let wasmModule;

async function initWasm() {
    try {
        console.log('Loading WASM module...');
        // Import the WASM module
        wasmModule = await import('./pkg/weather_lib.js');
        console.log('WASM module imported:', wasmModule);

        // Initialize the module
        await wasmModule.default();
        console.log('WASM module initialized successfully');

        // Check if the function exists
        if (typeof wasmModule.get_weather_data === 'function') {
            console.log('get_weather_data function is available');
        } else {
            console.error('get_weather_data function is not available');
            console.log('Available functions:', Object.keys(wasmModule));
        }
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        showError('Failed to load WASM module. Please check your build.');
    }
}

document.addEventListener('DOMContentLoaded', async () => {
    await initWasm();

    const form = document.getElementById('weatherForm');

    form.addEventListener('submit', async (e) => {
        e.preventDefault();

        const formData = new FormData(form);
        const request = {
            zip: formData.get('zip'),
            country: formData.get('country'),
            units: formData.get('units'),
            api_key: formData.get('apiKey')
        };

        showLoading();

        try {
            const response = await getWeatherData(request);
            if (response.error) {
                showError(response.error);
            } else {
                showResults(response);
            }
        } catch (error) {
            console.error('Error getting weather data:', error);
            showError(error.message);
        }
    });
});

async function getWeatherData(request) {
    if (!wasmModule) {
        throw new Error('WASM module not loaded');
    }

    if (typeof wasmModule.get_weather_data !== 'function') {
        throw new Error('get_weather_data function not available in WASM module');
    }

    try {
        console.log('Calling WASM function with:', request);
        const requestJson = JSON.stringify(request);
        const responseJson = await wasmModule.get_weather_data(requestJson);
        console.log('WASM response:', responseJson);
        return JSON.parse(responseJson);
    } catch (error) {
        console.error('WASM function error:', error);
        throw new Error(`Weather API error: ${error.message}`);
    }
}

function showLoading() {
    document.getElementById('loading').classList.remove('hidden');
    document.getElementById('results').classList.add('hidden');
    document.getElementById('error').classList.add('hidden');
    document.getElementById('submitBtn').disabled = true;
}

function showResults(response) {
    document.getElementById('loading').classList.add('hidden');
    document.getElementById('error').classList.add('hidden');
    document.getElementById('results').classList.remove('hidden');
    document.getElementById('submitBtn').disabled = false;

    // Display location information
    const locationDetails = document.getElementById('locationDetails');
    locationDetails.innerHTML = `
        <div class="detail-item">
            <span class="detail-label">Name:</span>
            <span class="detail-value">${response.location.name}</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Country:</span>
            <span class="detail-value">${response.location.country}</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">ZIP:</span>
            <span class="detail-value">${response.location.zip}</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Latitude:</span>
            <span class="detail-value">${response.location.lat.toFixed(4)}</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Longitude:</span>
            <span class="detail-value">${response.location.lon.toFixed(4)}</span>
        </div>
    `;

    // Display weather information
    const weatherData = JSON.parse(response.weather);
    const weatherDetails = document.getElementById('weatherDetails');
    weatherDetails.innerHTML = formatWeatherData(weatherData);
}

function formatWeatherData(weatherData) {
    return `
        <div class="detail-item">
            <span class="detail-label">Temperature:</span>
            <span class="detail-value">${weatherData.main?.temp || 'N/A'}</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Feels Like:</span>
            <span class="detail-value">${weatherData.main?.feels_like || 'N/A'}</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Humidity:</span>
            <span class="detail-value">${weatherData.main?.humidity || 'N/A'}%</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Pressure:</span>
            <span class="detail-value">${weatherData.main?.pressure || 'N/A'} hPa</span>
        </div>
        <div class="detail-item">
            <span class="detail-label">Description:</span>
            <span class="detail-value">${weatherData.weather?.[0]?.description || 'N/A'}</span>
        </div>
    `;
}

function showError(message) {
    document.getElementById('loading').classList.add('hidden');
    document.getElementById('results').classList.add('hidden');
    document.getElementById('error').classList.remove('hidden');
    document.getElementById('submitBtn').disabled = false;

    document.getElementById('errorMessage').textContent = message;
}