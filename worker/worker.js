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

  var cf = {};
  if (request.cf !== undefined) {
    for (var key of request.cf.keys()) {
      cf[key] = request.cf.get(key);
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
