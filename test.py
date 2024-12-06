import requests

base_url = "http://127.0.0.1:5005"
endpoints = [
    "/index.html",
    "/about.html",
    "/missing.html",
    "/img_1.jpg",
    "/invalid/path",
]

for endpoint in endpoints:
    url = f"{base_url}{endpoint}"
    print(f"\nTesting URL: {url}")
    try:
        response = requests.get(url)
        print(f"Status: {response.status_code}")
        print(f"Response: {response.text[:20]}")  
    except requests.RequestException as e:
        print(f"Error: {e}")

# Prueba de múltiples solicitudes rápidas
print("\nTesting multiple rapid requests...")
rapid_url = f"{base_url}/index.html"
for _ in range(5):
    try:
        response = requests.get(rapid_url)
        print(f"Request successful: {response.status_code}")
    except requests.RequestException as e:
        print(f"Request failed: {e}")
