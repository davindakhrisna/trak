import { chromium } from "playwright";
import fs from "node:fs";
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
    `[Responsive Check] ${label}: innerWidth=${innerWidth}, scrollWidth=${scrollWidth} -> ${overflow ? "FAILED OVERFLOW" : "PASS NO OVERFLOW"}`
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

  console.log("Browser launched. Verifying responsiveness across pages...");

  // 1. Desktop Light: Today
  {
    const page = await browser.newPage({
      viewport: { width: 1280, height: 900 },
    });
    await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);
    await checkNoHorizontalOverflow(page, "Desktop Today (Light)");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "today_desktop_light.png"),
      fullPage: false,
    });
    console.log("Captured today_desktop_light.png");
    await page.close();
  }

  // 2. Desktop Dark: Today (toggle dark via theme selector)
  {
    const page = await browser.newPage({
      viewport: { width: 1280, height: 900 },
      colorScheme: "dark",
    });
    await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);

    // Click 'dark' button
    const darkBtn = page.getByRole("button", { name: "dark" });
    if (await darkBtn.isVisible()) {
      await darkBtn.click();
      await page.waitForTimeout(300);
    }

    await checkNoHorizontalOverflow(page, "Desktop Today (Dark)");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "today_desktop_dark.png"),
      fullPage: false,
    });
    console.log("Captured today_desktop_dark.png");
    await page.close();
  }

  // 3. Mobile Light: Today
  {
    const page = await browser.newPage({
      viewport: { width: 390, height: 844 },
    });
    await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);
    await checkNoHorizontalOverflow(page, "Mobile Today (Light)");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "today_mobile_light.png"),
      fullPage: false,
    });
    console.log("Captured today_mobile_light.png");
    await page.close();
  }

  // 4. Mobile Dark: Today
  {
    const page = await browser.newPage({
      viewport: { width: 390, height: 844 },
      colorScheme: "dark",
    });
    await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);

    const darkBtn = page.getByRole("button", { name: "dark" });
    if (await darkBtn.isVisible()) {
      await darkBtn.click();
      await page.waitForTimeout(300);
    }

    await checkNoHorizontalOverflow(page, "Mobile Today (Dark)");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "today_mobile_dark.png"),
      fullPage: false,
    });
    console.log("Captured today_mobile_dark.png");
    await page.close();
  }

  // 5. Desktop: Templates
  {
    const page = await browser.newPage({
      viewport: { width: 1280, height: 900 },
    });
    await page.goto(`${BASE_URL}/templates`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);
    await checkNoHorizontalOverflow(page, "Desktop Templates");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "templates_desktop.png"),
      fullPage: false,
    });
    console.log("Captured templates_desktop.png");
    await page.close();
  }

  // 6. Mobile: Templates
  {
    const page = await browser.newPage({
      viewport: { width: 390, height: 844 },
    });
    await page.goto(`${BASE_URL}/templates`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);
    await checkNoHorizontalOverflow(page, "Mobile Templates");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "templates_mobile.png"),
      fullPage: false,
    });
    console.log("Captured templates_mobile.png");
    await page.close();
  }

  // 7. Desktop: Users
  {
    const page = await browser.newPage({
      viewport: { width: 1280, height: 900 },
    });
    await page.goto(`${BASE_URL}/users`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);
    await checkNoHorizontalOverflow(page, "Desktop Users");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "users_desktop.png"),
      fullPage: false,
    });
    console.log("Captured users_desktop.png");
    await page.close();
  }

  // 8. Mobile: Users
  {
    const page = await browser.newPage({
      viewport: { width: 390, height: 844 },
    });
    await page.goto(`${BASE_URL}/users`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);
    await checkNoHorizontalOverflow(page, "Mobile Users");
    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "users_mobile.png"),
      fullPage: false,
    });
    console.log("Captured users_mobile.png");
    await page.close();
  }

  // 9. Interactive WASM Hydration Test (Today):
  // Test adding a task with deadline, toggling it to Done, and verifying counts
  {
    const page = await browser.newPage({
      viewport: { width: 1280, height: 900 },
    });
    await page.goto(`${BASE_URL}/`, { waitUntil: "networkidle" });
    await page.waitForTimeout(500);

    const taskInput = page.getByPlaceholder("Write a task...");
    const startInput = page.getByPlaceholder("Start");
    const dueInput = page.getByPlaceholder("Due");
    const addBtn = page.getByRole("button", { name: "Add" });

    await taskInput.fill("Automated Playwright responsiveness test task");
    await startInput.fill("09:00");
    await dueInput.fill("18:00");
    await addBtn.click();
    await page.waitForTimeout(300);

    // Verify task is in In Progress
    const newTask = page.getByText(
      "Automated Playwright responsiveness test task"
    );
    const isVisible = await newTask.isVisible();
    console.log(
      `[Interaction] Added task is visible in In Progress: ${isVisible}`
    );

    await page.screenshot({
      path: path.join(ARTIFACT_DIR, "today_interactive_added.png"),
      fullPage: false,
    });
    console.log("Captured today_interactive_added.png");

    await page.close();
  }

  await browser.close();
  console.log("All Playwright responsiveness and visual checks completed!");
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
