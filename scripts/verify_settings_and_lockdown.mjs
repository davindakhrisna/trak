import { chromium } from "playwright";
import path from "node:path";

const ARTIFACT_DIR =
  "/home/kryisnn/.gemini/antigravity-cli/brain/70bf0151-b3ef-4d82-b5e6-d75ace6bc29a";
const BASE_URL = "http://127.0.0.1:3000";

const chromePath = path.join(
  process.env.PLAYWRIGHT_BROWSERS_PATH || "",
  "chromium-1248/chrome-linux64/chrome"
);

async function checkNoHorizontalOverflow(page, label) {
  const scrollWidth = await page.evaluate(
    () => document.documentElement.scrollWidth
  );
  const innerWidth = await page.evaluate(() => window.innerWidth);
  const overflow = scrollWidth > innerWidth;
  console.log(
    `[Responsive] ${label}: innerWidth=${innerWidth}, scrollWidth=${scrollWidth} -> ${overflow ? "FAILED OVERFLOW" : "PASS"}`
  );
  if (overflow) {
    throw new Error(`Horizontal overflow detected in ${label}!`);
  }
}

async function run() {
  const browser = await chromium.launch({
    executablePath: chromePath,
    headless: true,
  });

  const context = await browser.newContext({
    viewport: { width: 1280, height: 950 },
  });
  const page = await context.newPage();

  console.log("=== 1. Check /register or /login to establish session ===");
  await page.goto(`${BASE_URL}/register`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);

  const adminInput = page.locator('input[placeholder="Root admin username"]');
  if (await adminInput.isVisible()) {
    await adminInput.fill("admin");
    await page.fill('input[placeholder="Password (min 4 chars)"]', "password123");
    await page.click('button[type="submit"]');
    await page.waitForURL(`${BASE_URL}/`, { timeout: 6000 });
  } else {
    await page.goto(`${BASE_URL}/login`, { waitUntil: "networkidle" });
    await page.fill('input[placeholder="Username"]', "admin");
    await page.fill('input[placeholder="Password"]', "password123");
    await page.click('button[type="submit"]');
    await page.waitForURL(`${BASE_URL}/`, { timeout: 6000 });
  }

  console.log("=== 2. Navigate to /settings ===");
  await page.goto(`${BASE_URL}/settings`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);
  await checkNoHorizontalOverflow(page, "Desktop Settings (Light)");

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "settings_desktop_light.png"),
  });
  console.log("Captured settings_desktop_light.png");

  console.log("=== 3. Verify AdGuard Home & Interactive Controls ===");
  // Test AdGuard input
  const adguardUrl = page.locator('input[placeholder="http://192.168.1.100:80"]');
  await adguardUrl.fill("http://192.168.1.50:8080");

  // Toggle Instagram checkbox
  await page.click('span:has-text("Instagram")');
  await page.waitForTimeout(200);

  // Click Test AdGuard Connection
  await page.click('button:has-text("Test AdGuard Connection")');
  await page.waitForTimeout(600);

  // Click Save All Settings
  await page.click('button:has-text("Save All Settings")');
  await page.waitForTimeout(500);

  // Test Guide Tabs switching
  await page.click('button:has-text("AdGuard Home")');
  await page.waitForTimeout(300);
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "settings_guide_tabs_adguard.png"),
  });
  console.log("Captured settings_guide_tabs_adguard.png");

  await page.click('button:has-text("Android (MacroDroid)")');
  await page.waitForTimeout(300);

  await page.click('button:has-text("Ntfy Pairing")');
  await page.waitForTimeout(300);
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "settings_guide_tabs_ntfy.png"),
  });
  console.log("Captured settings_guide_tabs_ntfy.png");

  // Read the private lockdown URL
  const apiDiv = page.locator('div:has-text("/api/lockdown/")').last();
  const fullApiUrl = (await apiDiv.textContent()).trim();
  console.log("Discovered Lockdown API Endpoint:", fullApiUrl);

  // Verify the HTTP endpoint directly
  const response = await page.request.get(fullApiUrl);
  console.log("API Status:", response.status());
  const json = await response.json();
  console.log("API Response JSON:", json);

  if (response.status() !== 200 || json.locked !== false) {
    throw new Error("Lockdown API returned unexpected result!");
  }

  console.log("=== 4. Dark Mode Settings View ===");
  const darkBtn = page.getByRole("button", { name: "dark" });
  if (await darkBtn.isVisible()) {
    await darkBtn.click();
    await page.waitForTimeout(300);
  }
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "settings_desktop_dark.png"),
  });
  console.log("Captured settings_desktop_dark.png");

  console.log("=== 5. Mobile Responsiveness Check (390px) ===");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForTimeout(400);
  await checkNoHorizontalOverflow(page, "Mobile Settings (390px)");

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "settings_mobile_light.png"),
  });
  console.log("Captured settings_mobile_light.png");

  await browser.close();
  console.log("🎉 SETTINGS & LOCKDOWN VERIFICATION PASSED COMPLETELY!");
}

run().catch((err) => {
  console.error("Test failed:", err);
  process.exit(1);
});
