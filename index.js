async function executeSearch(query) {
  console.log("Querying /api/search...");
  const response = await fetch("/api/search", {
    method: "POST",
    headers: {
      "Content-Type": "text/plain",
    },
    body: query,
  });
  const results = await response.json();

  const resultsDiv = document.getElementById('results');
  resultsDiv.innerHTML = '';

  const elements = Object.entries(results).sort(([, a], [, b]) => b - a).slice(0, 20);
  for (const [path, score] of elements) {
    const item = document.createElement('div');
    item.className = 'item';
    item.textContent = `${path} (Score: ${score.toFixed(3)})`;
    resultsDiv.appendChild(item);
  }
}
