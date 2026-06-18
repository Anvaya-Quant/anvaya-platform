import { test, expect } from '@playwright/test';

test('can add Hadamard gate and see probabilities', async ({ page }) => {
  await page.goto('/');
  await page.waitForSelector('text=ANVAYA Studio', { timeout: 30000 });
  await page.waitForFunction(() => !document.body.innerText.includes('Loading ANVAYA Core'));

  await page.fill('input[type="number"]', '1');

  await page.evaluate(() => {
    const store = (window as any).__ANVAYA_STORE__;
    store.getState().addGate({ id: 'test-h', gate: 'h', targets: [0] });
  });

  await page.click('button:has-text("Simulate")');

  const bars = page.locator('[data-testid="probability-bar"]');
  await expect(bars).toHaveCount(2, { timeout: 15000 });

  const firstPercent = await bars.nth(0).locator('span').first().textContent();
  expect(firstPercent).toMatch(/50\.\d?%|49\.\d?%|51\.\d?%/);
  const secondPercent = await bars.nth(1).locator('span').first().textContent();
  expect(secondPercent).toMatch(/50\.\d?%|49\.\d?%|51\.\d?%/);
});

test('optimizer removes redundant X gates', async ({ page }) => {
  await page.goto('/');
  await page.waitForFunction(() => !document.body.innerText.includes('Loading ANVAYA Core'));

  await page.fill('input[type="number"]', '1');

  await page.evaluate(() => {
    const store = (window as any).__ANVAYA_STORE__;
    store.getState().addGate({ id: 'test-x1', gate: 'x', targets: [0] });
    store.getState().addGate({ id: 'test-x2', gate: 'x', targets: [0] });
  });

  const gateNodes = page.locator('[data-testid="gate-node"]');
  await expect(gateNodes).toHaveCount(2, { timeout: 15000 });

  await page.click('button:has-text("Optimize")');

  await expect(gateNodes).toHaveCount(0, { timeout: 15000 });
});
