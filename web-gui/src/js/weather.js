import { WeatherService } from './weather-service.js';
import { LocationService } from './location-service.js';

export class WeatherApp {
    constructor() {
        this.init();
    }

    async init() {
        try {
            // Load WASM module
            const wasm = await import('../../wasm/weather.js');
            await wasm.default();
            
            // Initialize services
            this.weatherService = new WeatherService(wasm);
            this.locationService = new LocationService(wasm);
            
            // Set up event listeners
            this.setupEventListeners();
            
        } catch (error) {
            console.error('Failed to initialize app:', error);
        }
    }

    setupEventListeners() {
        const form = document.getElementById('searchForm');
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            const query = document.getElementById('locationInput').value.trim();
            if (query) {
                this.handleSearch(query);
            }
        });
    }

    async handleSearch(query) {
        this.showLoading(true);
        this.hideError();
        
        try {
            const location = await this.locationService.getLocation(query);
            const weather = await this.weatherService.getWeather(location.lat, location.lon);
            
            this.displayWeather(weather, location);
            
        } catch (error) {
            this.showError(error.message);
        } finally {
            this.showLoading(false);
        }
    }

    showLoading(show) {
        document.getElementById('loadingIndicator').classList.toggle('d-none', !show);
    }

    showError(message) {
        const errorDiv = document.getElementById('errorDisplay');
        const errorMsg = document.getElementById('errorMessage');
        errorMsg.textContent = message;
        errorDiv.classList.remove('d-none');
    }

    hideError() {
        document.getElementById('errorDisplay').classList.add('d-none');
    }

    displayWeather(weather, location) {
        const weatherDiv = document.getElementById('weatherDisplay');
        
        // Create weather card HTML
        weatherDiv.innerHTML = `
            <div class="card weather-card">
                <div class="card-body">
                    <h3>${weather.name}</h3>
                    <div class="temperature">${weather.main.temp.toFixed(1)}°C</div>
                    <p>${weather.weather[0].description}</p>
                    <div class="weather-details">
                        <div class="detail-item">
                            <span>Feels like:</span>
                            <span>${weather.main.feels_like.toFixed(1)}°C</span>
                        </div>
                        <div class="detail-item">
                            <span>Humidity:</span>
                            <span>${weather.main.humidity}%</span>
                        </div>
                        <div class="detail-item">
                            <span>Wind:</span>
                            <span>${weather.wind.speed} m/s</span>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        weatherDiv.classList.remove('d-none');
    }
}