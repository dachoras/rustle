const CACHE_NAME = "RUSTLE_PWA_CACHE_V2";
const ASSETS_TO_CACHE = [];

const preload = async () => {
  console.log("Installing web app");
  const cache = await caches.open(CACHE_NAME);
  console.log("Caching index and important routes");
  try {
    const response = await fetch("/asset-manifest.json");
    const assets = await response.json();
    ASSETS_TO_CACHE.push(...assets);
  } catch (err) {
    console.warn("Failed to fetch asset manifest, falling back to basic caching:", err);
    ASSETS_TO_CACHE.push("/", "/index.html", "/public/favicon.png", "/public/manifest.json");
  }
  console.log("Assets Cached:", ASSETS_TO_CACHE);
  return cache.addAll(ASSETS_TO_CACHE);
};

globalThis.addEventListener("install", (event) => {
  event.waitUntil(preload().then(() => globalThis.skipWaiting()));
});

globalThis.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== CACHE_NAME) {
            console.log("Deleting old cache:", cacheName);
            return caches.delete(cacheName);
          }
        })
      );
    }).then(() => clients.claim())
  );
});

globalThis.addEventListener("fetch", (event) => {
  if (!event.request.url.startsWith('http')) return;

  event.respondWith(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.match(event.request).then((cachedResponse) => {
        const fetchPromise = fetch(event.request).then((networkResponse) => {
          if (networkResponse.status === 200) {
            cache.put(event.request, networkResponse.clone());
          }
          return networkResponse;
        }).catch((err) => {
          console.warn("SW fetch failed:", err);
        });
        return cachedResponse || fetchPromise;
      });
    })
  );
});
