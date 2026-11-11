import { chromium } from "playwright";
import path from "node:path";

const chromePath = path.join(
  process.env.PLAYWRIGHT_BROWSERS_PATH || "",
  "chromium-1248/chrome-linux64/chrome"
);

async function main() {
  const browser = await chromium.launch({
    executablePath: chromePath,
    headless: true,
  });

  const viewports = [
    { name: "mobile_375", width: 375, height: 667 },
    { name: "tablet_640", width: 640, height: 900 },
    { name: "tablet_768", width: 768, height: 1024 },
    { name: "tablet_820", width: 820, height: 1180 },
    { name: "laptop_1024", width: 1024, height: 768 },
    { name: "desktop_1280", width: 1280, height: 800 },
  ];

  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const page = await context.newPage();

  // Reset or ensure admin registered
  await page.goto("http://127.0.0.1:3000/register", { waitUntil: "networkidle" });
  if (await page.locator('input[placeholder="Root admin username"]').isVisible()) {
    await page.fill('input[placeholder="Root admin username"]', "admin");
    await page.fill('input[placeholder="Password (min 4 chars)"]', "adminpassword123");
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);
  } else {
    await page.goto("http://127.0.0.1:3000/login", { waitUntil: "networkidle" });
    await page.fill('input[placeholder="Username"]', "admin");
    await page.fill('input[placeholder="Password"]', "adminpassword123");
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);
  }

  // Go to Today
  await page.goto("http://127.0.0.1:3000/", { waitUntil: "networkidle" });
  await page.waitForTimeout(600);

  // Add sample tasks if none
  const taskInput = page.locator('input[placeholder="Write a task..."]');
  if (await taskInput.isVisible()) {
    await taskInput.fill("Sample responsive testing task item");
    await page.locator('input[placeholder="Start"]').fill("09:00");
    await page.locator('input[placeholder="Due"]').fill("17:00");
    await page.click('button:has-text("Add")');
    await page.waitForTimeout(500);
  }

  // Add a sample template as well
  await page.goto("http://127.0.0.1:3000/templates", { waitUntil: "networkidle" });
  await page.waitForTimeout(600);
  const tplInput = page.locator('input[placeholder="New recurring task..."]');
  if (await tplInput.isVisible()) {
    await tplInput.fill("Weekly engineering retrospective and demo");
    await page.locator('input[placeholder="Start"]').fill("14:00");
    await page.locator('input[placeholder="Due"]').fill("15:30");
    await page.click('button:has-text("Save")');
    await page.waitForTimeout(500);
  }

  for (const vp of viewports) {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.waitForTimeout(400);

    // Today Page
    await page.goto("http://127.0.0.1:3000/", { waitUntil: "networkidle" });
    await page.waitForTimeout(400);
    await page.screenshot({ path: `/home/kryisnn/.gemini/antigravity-cli/brain/70bf0151-b3ef-4d82-b5e6-d75ace6bc29a/diag_today_${vp.name}.png` });

    // Templates Page
    await page.goto("http://127.0.0.1:3000/templates", { waitUntil: "networkidle" });
    await page.waitForTimeout(400);
    await page.screenshot({ path: `/home/kryisnn/.gemini/antigravity-cli/brain/70bf0151-b3ef-4d82-b5e6-d75ace6bc29a/diag_templates_${vp.name}.png` });

    console.log(`Captured ${vp.name} (${vp.width}x${vp.height})`);
  }

  await browser.close();
  console.log("All screenshots captured cleanly!");
}

main().catch(console.error);
