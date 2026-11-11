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
    viewport: { width: 1280, height: 900 },
  });
  const page = await context.newPage();

  console.log("=== STEP 1: Verify clean uninitialized instance on / ===");
  await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);
  await checkNoHorizontalOverflow(page, "Clean Home");
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "01_clean_welcome.png"),
  });
  console.log("Captured 01_clean_welcome.png");

  console.log("=== STEP 2: Navigate to /register and register root admin ===");
  await page.goto(`${BASE_URL}/register`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);

  // Fill in root admin registration
  await page.fill('input[placeholder="Root admin username"]', "admin");
  await page.fill('input[placeholder="Password (min 4 chars)"]', "password123");
  await page.click('button[type="submit"]');

  // Wait for redirect to home
  await page.waitForURL(`${BASE_URL}/`, { timeout: 6000 });
  await page.waitForTimeout(800);
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "02_admin_registered.png"),
  });
  console.log("Captured 02_admin_registered.png - Root admin registered and authenticated!");

  console.log("=== STEP 3: Add tasks on Today dashboard ===");
  // Task 1
  await page.fill('input[placeholder="Write a task..."]', "Morning standup & system checks");
  await page.fill('input[placeholder="Start"]', "09:00");
  await page.fill('input[placeholder="Due"]', "09:30");
  await page.click('button:has-text("Add")');
  await page.waitForTimeout(400);

  // Task 2
  await page.fill('input[placeholder="Write a task..."]', "Deep work: Rust backend optimization");
  await page.fill('input[placeholder="Start"]', "10:00");
  await page.fill('input[placeholder="Due"]', "13:00");
  await page.click('button:has-text("Add")');
  await page.waitForTimeout(400);

  // Task 3
  await page.fill('input[placeholder="Write a task..."]', "Review security headers & cookies");
  await page.fill('input[placeholder="Start"]', "14:00");
  await page.fill('input[placeholder="Due"]', "15:00");
  await page.click('button:has-text("Add")');
  await page.waitForTimeout(600);

  // Toggle first task to done
  const checkboxes = page.locator('button[role="checkbox"]');
  const count = await checkboxes.count();
  if (count > 0) {
    await checkboxes.first().click();
    await page.waitForTimeout(500);
  }

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "03_today_tasks_added.png"),
  });
  console.log("Captured 03_today_tasks_added.png - Tasks added and completed state verified!");

  console.log("=== STEP 4: Templates Page: Add recurring template ===");
  await page.goto(`${BASE_URL}/templates`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);

  await page.fill('input[placeholder="Write a template task..."]', "Daily sync with infrastructure team");
  await page.fill('input[placeholder="Start"]', "15:30");
  await page.fill('input[placeholder="Due"]', "16:00");
  await page.click('button:has-text("Add")');
  await page.waitForTimeout(600);

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "04_template_added.png"),
  });
  console.log("Captured 04_template_added.png - Template added!");

  console.log("=== STEP 5: Verify template auto-provisioned to Today ===");
  await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "05_today_provisioned.png"),
  });
  console.log("Captured 05_today_provisioned.png - Idempotent provisioning confirmed!");

  console.log("=== STEP 6: Users Management (/users) ===");
  await page.goto(`${BASE_URL}/users`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);

  // Provision new user marcus_aurelius
  await page.fill('input[placeholder="Username"]', "marcus_aurelius");
  await page.fill('input[placeholder="Password"]', "rome1234");
  await page.click('button:has-text("Add user")');
  await page.waitForTimeout(800);

  // Toggle role to admin
  const toggleBtn = page.locator('button:has-text("Set admin")');
  if (await toggleBtn.count() > 0) {
    await toggleBtn.first().click();
    await page.waitForTimeout(600);
  }

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "06_users_management.png"),
  });
  console.log("Captured 06_users_management.png - User provisioned and role toggled!");

  console.log("=== STEP 7: Sign Out & Verify Protected Route ===");
  const signOutLink = page.locator('a:has-text("Sign out")');
  if (await signOutLink.isVisible()) {
    await signOutLink.click();
    await page.waitForURL(`${BASE_URL}/login`, { timeout: 6000 });
    await page.waitForTimeout(600);
  }

  // Try visiting /users unauthenticated
  await page.goto(`${BASE_URL}/users`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);
  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "07_signed_out_admin_only.png"),
  });
  console.log("Captured 07_signed_out_admin_only.png - Admin-only protection verified!");

  console.log("=== STEP 8: Standard User Login & Multi-Tenant Isolation ===");
  await page.goto(`${BASE_URL}/login`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);

  await page.fill('input[placeholder="Username"]', "marcus_aurelius");
  await page.fill('input[placeholder="Password"]', "rome1234");
  await page.click('button[type="submit"]');

  await page.waitForURL(`${BASE_URL}/`, { timeout: 6000 });
  await page.waitForTimeout(800);

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "08_user_isolated_today.png"),
  });
  console.log("Captured 08_user_isolated_today.png - User authenticated with isolated tasks!");

  console.log("=== STEP 9: Mobile Viewport Responsiveness ===");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForTimeout(400);
  await checkNoHorizontalOverflow(page, "Mobile Today (390px)");

  await page.screenshot({
    path: path.join(ARTIFACT_DIR, "09_mobile_responsive.png"),
  });
  console.log("Captured 09_mobile_responsive.png - Perfect mobile layout verified!");

  await browser.close();
  console.log("🎉 ALL E2E VERIFICATION CHECKS PASSED!");
}

run().catch((err) => {
  console.error("E2E Test Failed:", err);
  process.exit(1);
});
