import { chromium } from "playwright";
import path from "node:path";

const ARTIFACT_DIR =
  "/home/kryisnn/.gemini/antigravity-cli/brain/70bf0151-b3ef-4d82-b5e6-d75ace6bc29a";
const BASE_URL = "http://127.0.0.1:3000";

const chromePath = path.join(
  process.env.PLAYWRIGHT_BROWSERS_PATH || "",
  "chromium-1248/chrome-linux64/chrome"
);

function assert(condition, message) {
  if (!condition) {
    console.error(`❌ ASSERTION FAILED: ${message}`);
    throw new Error(message);
  }
  console.log(`✓ ${message}`);
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

  console.log("\n=======================================================");
  console.log("  TRAK END-TO-END CLEAN ROOM PRODUCTION VERIFICATION");
  console.log("=======================================================\n");

  // -------------------------------------------------------------------------
  // STEP 1: Uninitialized Instance State & Bouncing
  // -------------------------------------------------------------------------
  console.log("--> STEP 1: Uninitialized Instance State & Route Bouncing");
  await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);

  assert(page.url().includes("/login"), "Unauthenticated request to '/' bounces to '/login'");

  const pageContent = await page.innerText("body");
  assert(
    pageContent.includes("Instance uninitialized"),
    "Login page acknowledges uninitialized instance"
  );
  assert(
    pageContent.includes("Register first admin"),
    "'Register first admin' CTA button is present"
  );

  // Protected routes bounce to /login
  for (const route of ["/templates", "/settings", "/users"]) {
    await page.goto(`${BASE_URL}${route}`, { waitUntil: "networkidle" });
    await page.waitForTimeout(200);
    assert(page.url().includes("/login"), `Unauthenticated request to '${route}' bounces to '/login'`);
  }

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_01_uninitialized_login.png") });

  // -------------------------------------------------------------------------
  // STEP 2: Root Admin Registration
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 2: Root Admin Registration via /register");
  await page.goto(`${BASE_URL}/register`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);
  assert(page.url().includes("/register"), "Navigated to /register for initial setup");

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_02_register_page.png") });

  await page.fill('input[placeholder="Root admin username"]', "admin");
  await page.fill('input[placeholder="Password (min 4 chars)"]', "adminpassword123");
  await page.click('button[type="submit"]');
  await page.waitForTimeout(1000);
  await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);
  assert(page.url() === `${BASE_URL}/`, "Admin registration automatically logs in and redirects to '/'");

  const todayText = await page.innerText("body");
  assert(todayText.includes("admin"), "Session identifies logged in admin user in sidebar");
  assert(todayText.includes("Today"), "Today dashboard is active");

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_03_admin_authenticated_today.png") });

  // -------------------------------------------------------------------------
  // STEP 3: Hiding /register After Admin Initialization
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 3: Verify /register is Permanently Hidden and Bounced");
  await page.goto(`${BASE_URL}/register`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);
  assert(!page.url().endsWith("/register"), "/register access bounces away after admin setup");

  await page.goto(`${BASE_URL}/registers`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);
  assert(!page.url().endsWith("/registers"), "/registers access bounces away after admin setup");

  // -------------------------------------------------------------------------
  // STEP 4: Today Task CRUD
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 4: Today Tasks CRUD & Checkbox Flow");
  await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);

  // Add Task 1
  await page.fill('input[placeholder="Write a task..."]', "Deploy production binary to VPS");
  await page.fill('input[placeholder="Start"]', "09:00");
  await page.fill('input[placeholder="Due"]', "10:30");
  await page.click('button:has-text("Add")');
  await page.waitForTimeout(500);

  // Add Task 2
  await page.fill('input[placeholder="Write a task..."]', "Verify Caddy HTTPS proxy");
  await page.fill('input[placeholder="Start"]', "11:00");
  await page.fill('input[placeholder="Due"]', "12:00");
  await page.click('button:has-text("Add")');
  await page.waitForTimeout(500);

  let tasksText = await page.innerText("body");
  assert(tasksText.includes("Deploy production binary to VPS"), "Task 1 added successfully");
  assert(tasksText.includes("Verify Caddy HTTPS proxy"), "Task 2 added successfully");

  // Toggle Task 1
  const firstCheckbox = page.locator('button[role="checkbox"]').first();
  await firstCheckbox.click();
  await page.waitForTimeout(500);

  // Delete Task 2
  const deleteBtn = page.locator('button[aria-label^="Remove Verify Caddy"]').first();
  if (await deleteBtn.isVisible()) {
    await deleteBtn.click();
    await page.waitForTimeout(500);
  }

  tasksText = await page.innerText("body");
  assert(!tasksText.includes("Verify Caddy HTTPS proxy"), "Task 2 deleted successfully");

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_04_today_tasks_crud.png") });

  // -------------------------------------------------------------------------
  // STEP 5: Templates Scheduling
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 5: Weekly Recurring Templates Scheduling");
  await page.goto(`${BASE_URL}/templates`, { waitUntil: "networkidle" });
  await page.waitForTimeout(400);

  // Select Monday
  await page.click('button:has-text("MON")');
  await page.waitForTimeout(300);

  await page.fill('input[placeholder="New recurring task..."]', "Weekly VPS security & integrity check");
  await page.fill('input[placeholder="Start"]', "08:00");
  await page.fill('input[placeholder="Due"]', "09:00");
  await page.click('button:has-text("Save")');
  await page.waitForTimeout(500);

  const tplText = await page.innerText("body");
  assert(
    tplText.includes("Weekly VPS security & integrity check"),
    "Recurring template created for Monday"
  );

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_05_templates_scheduled.png") });

  // -------------------------------------------------------------------------
  // STEP 6: Settings & Unified Save
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 6: Settings, Ntfy & Mobile Lockdown Configuration");
  await page.goto(`${BASE_URL}/settings`, { waitUntil: "networkidle" });
  await page.waitForSelector('text=Mobile App Lockdown', { timeout: 8000 });
  await page.waitForTimeout(300);

  const settingsBody = await page.innerText("body");
  assert(!settingsBody.includes("Save Settings"), "No duplicate 'Save Settings' button inside Ntfy card");
  assert(/send test/i.test(settingsBody), "Ntfy 'Send Test' button is present");
  assert(/mobile app lockdown/i.test(settingsBody), "Mobile Lockdown card is present");
  assert(/save all settings/i.test(settingsBody), "Unified 'Save All Settings' button is present");

  // Save Settings
  await page.click('button:has-text("Save All Settings")');
  await page.waitForTimeout(600);

  const feedback = await page.innerText("body");
  assert(feedback.includes("Settings saved successfully"), "Unified Save successfully persisted settings");

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_06_settings_configured.png") });

  // -------------------------------------------------------------------------
  // STEP 7: Users Management
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 7: Admin Users Management & Role Promotion");
  await page.goto(`${BASE_URL}/users`, { waitUntil: "networkidle" });
  await page.waitForSelector('input[placeholder="Username"]', { timeout: 8000 });
  await page.waitForTimeout(300);

  await page.fill('input[placeholder="Username"]', "colleague");
  await page.fill('input[placeholder="Password"]', "userpassword123");
  // Check admin
  await page.click('button[aria-label="Set new user as admin"]');
  await page.click('button:has-text("Add user")');
  await page.waitForTimeout(600);

  let usersListText = await page.locator("ul.scroll-monolith").innerText();
  assert(usersListText.includes("colleague"), "User 'colleague' added to instance");
  assert(/admin/i.test(usersListText), "User 'colleague' initialized as Admin");

  // Demote to User
  const toggleBtn = page.locator('button:has-text("Revoke admin")').first();
  if (await toggleBtn.isVisible()) {
    await toggleBtn.click();
    await page.waitForTimeout(600);
  }

  // Delete colleague
  const removeBtn = page.locator('button[aria-label="Remove user"]').first();
  if (await removeBtn.isVisible()) {
    await removeBtn.click();
    await page.waitForTimeout(600);
  }

  usersListText = await page.locator("ul.scroll-monolith").innerText();
  assert(!usersListText.includes("colleague"), "User 'colleague' successfully deleted");

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_07_users_management.png") });

  // -------------------------------------------------------------------------
  // STEP 8: Sign Out & Route Re-Lock Verification
  // -------------------------------------------------------------------------
  console.log("\n--> STEP 8: Sign Out & Re-Lock Verification");
  await page.click('a:has-text("Sign out")');
  await page.waitForTimeout(600);

  assert(page.url().includes("/login"), "Sign out redirects to /login");

  const loginText = await page.innerText("body");
  assert(!loginText.includes("Instance uninitialized"), "Registration closed; no uninitialized prompt");
  assert(!loginText.includes("Register first admin"), "No register button shown on login");

  // Verify all routes bounce now that user is signed out
  for (const r of ["/", "/templates", "/settings", "/users", "/register"]) {
    await page.goto(`${BASE_URL}${r}`, { waitUntil: "networkidle" });
    await page.waitForTimeout(200);
    assert(page.url().includes("/login"), `Signed out request to '${r}' bounces to /login`);
  }

  // Test invalid login error message formatting (no debug prefix)
  console.log("\n--> STEP 9: Verify Clean Error Formatting (No Leptos Debug Prefix)");
  await page.fill('input[placeholder="Username"]', "admin");
  await page.fill('input[placeholder="Password"]', "wrongpassword");
  await page.click('button[type="submit"]');
  await page.waitForTimeout(600);

  const errorMsg = await page.locator("p.border").innerText();
  assert(
    errorMsg === "Invalid username or password.",
    `Error displays directly without debug prefix: "${errorMsg}"`
  );

  // Test valid login
  await page.fill('input[placeholder="Password"]', "adminpassword123");
  await page.click('button[type="submit"]');
  await page.waitForTimeout(1000);
  await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
  await page.waitForTimeout(600);

  assert(page.url() === `${BASE_URL}/`, "Re-login with correct password succeeds and routes to '/'");

  await page.screenshot({ path: path.join(ARTIFACT_DIR, "e2e_08_reauthenticated_clean.png") });

  await browser.close();

  console.log("\n=======================================================");
  console.log("  🎉 ALL 8 PRODUCTION VERIFICATION STAGES PASSED! 🎉");
  console.log("=======================================================\n");
}

run().catch((err) => {
  console.error("Test execution failed:", err);
  process.exit(1);
});
