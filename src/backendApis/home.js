import { http } from '@tauri-apps/api'

const backendBaseUrl = process.env.VUE_APP_BACKEND_BASE_URL

export function apiTest(data) {
    return http.fetch(`${backendBaseUrl}/api/test`, {
        method: 'POST',
        body: http.Body.json(data)
    })
}
