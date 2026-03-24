import { test, expect } from '@playwright/test';

test('Verify Library grid and Detail Panel open', async ({ page }) => {
  await page.goto('http://localhost:1420/');

  // Wait for the main page to load
  await expect(page.locator('.sidebar')).toBeVisible({ timeout: 10000 });

  // Click the first item
  const firstItem = page.locator('div[style*="background-image: url"]').first();
  await expect(firstItem).toBeVisible();

  await firstItem.click();

  // Wait briefly for animations to complete, then take a screenshot
  await page.waitForTimeout(1000);
  await page.screenshot({ path: '/home/jules/verification/library_with_detail.png', fullPage: true });

  // Detail panel should have a title
  await expect(page.locator('h2').filter({ hasText: /.*/ })).toBeVisible();
});
