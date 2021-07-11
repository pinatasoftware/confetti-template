addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { confetti } = wasm_bindgen;
  await wasm_bindgen(wasm);

  var body;
  if (request.body) {
    body = await request.text();
  } else {
    body = "";
  }

  var headers = {};
  for (var key of request.headers.keys()) {
    headers[key] = request.headers.get(key);
  }

  let cf = {};
  if (request.cf !== undefined) {
    for (let key in request.cf) {
      // confetti seems to treat cf as a HashMap<String, String>, even though it has nested objects
      cf[key] = JSON.stringify(request.cf[key]);
    }
  }

  const response = await confetti({
    url: request.url,
    body: body,
    method: request.method,
    headers: headers,
    cf: cf,
  });

  return new Response(response.body, {
    status: response.status,
    headers: response.headers,
  });
}
