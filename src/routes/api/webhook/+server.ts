import { Webhooks } from "@octokit/webhooks";
import type { RequestHandler } from "@sveltejs/kit";

const WEBHOOK = new Webhooks({
  secret: (Bun || process).env.VITE_GITHUB_WEBHOOK_SECRET,
});
export const POST: RequestHandler = async ({ request }) => {
  return new Response(null, { status: 200 });
};
