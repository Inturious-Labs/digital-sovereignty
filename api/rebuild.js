// Triggered daily by Vercel Cron (see vercel.json).
//
// Why this exists: articles are committed with future dates. The production
// build runs `hugo --minify --gc` WITHOUT the -F flag, so Hugo omits any post
// whose date is still in the future. A rebuild therefore publishes exactly the
// articles whose date has arrived -- nothing sooner.
//
// Buttondown polls the RSS feed every 30 minutes and dedupes on <guid>
// (the article permalink), so a rebuild that changes nothing sends nothing.

export default async function handler(request, response) {
  const authHeader = request.headers.authorization;
  const cronSecret = process.env.CRON_SECRET;

  if (!cronSecret || authHeader !== `Bearer ${cronSecret}`) {
    return response.status(401).json({ ok: false, error: 'unauthorized' });
  }

  const hook = process.env.DEPLOY_HOOK_URL;
  if (!hook) {
    return response.status(500).json({ ok: false, error: 'DEPLOY_HOOK_URL not set' });
  }

  try {
    const res = await fetch(hook, { method: 'POST' });
    if (!res.ok) {
      return response
        .status(502)
        .json({ ok: false, error: `deploy hook returned ${res.status}` });
    }
    return response.status(200).json({ ok: true, triggered: new Date().toISOString() });
  } catch (err) {
    return response.status(502).json({ ok: false, error: String(err) });
  }
}
