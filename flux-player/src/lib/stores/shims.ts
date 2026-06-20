// @ts-nocheck
import { mock } from "bun:test";

// ── Mock localStorage ────────────────────────────────────────────────────────
const storage = new Map<string, string>();
const localStorageMock = {
  getItem: (key: string) => storage.get(key) ?? null,
  setItem: (key: string, value: string) => { storage.set(key, String(value)); },
  removeItem: (key: string) => { storage.delete(key); },
  clear: () => { storage.clear(); },
  get length() { return storage.size; },
  key: (index: number) => Array.from(storage.keys())[index] ?? null,
};

globalThis.localStorage = localStorageMock as unknown as Storage;

// ── Svelte Store Implementation ──────────────────────────────────────────────
export function writable(value: any) {
  const callbacks = new Set<(v: any) => void>();
  return {
    subscribe: (cb: (v: any) => void) => {
      callbacks.add(cb);
      cb(value);
      return () => {
        callbacks.delete(cb);
      };
    },
    set: (v: any) => {
      value = v;
      callbacks.forEach(cb => cb(value));
    },
    update: (fn: (v: any) => any) => {
      value = fn(value);
      callbacks.forEach(cb => cb(value));
    }
  };
}

export function get(store: any) {
  let value: any;
  const unsubscribe = store.subscribe((v: any) => value = v);
  if (typeof unsubscribe === 'function') unsubscribe();
  return value;
}

export function readable(value: any, start: (set: (v: any) => void) => (() => void) | void = () => {}) {
  const { subscribe, set } = writable(value);
  let stop: (() => void) | void;
  let subscribers = 0;
  return {
    subscribe: (cb: (v: any) => void) => {
      if (subscribers === 0) {
        stop = start(set);
      }
      subscribers++;
      const unsub = subscribe(cb);
      return () => {
        unsub();
        subscribers--;
        if (subscribers === 0 && stop) {
          stop();
        }
      };
    }
  };
}

export function derived(stores: any, fn: any, initial_value?: any) {
  const single = !Array.isArray(stores);
  const stores_array = single ? [stores] : stores;

  return readable(initial_value, (set) => {
    const values = new Array(stores_array.length);
    let pending = stores_array.length;
    const sync = () => {
      if (pending > 0) return;
      const res = single ? fn(values[0], set) : fn(values, set);
      if (res !== undefined && typeof res !== 'function') {
        set(res);
      }
    };

    const unsubs = stores_array.map((store, i) => store.subscribe((val: any) => {
      if (values[i] === undefined) pending--;
      values[i] = val;
      sync();
    }));

    return () => unsubs.forEach(u => u());
  });
}

// ── Module Mocks ─────────────────────────────────────────────────────────────
mock.module("svelte/store", () => ({
  writable,
  get,
  readable,
  derived,
}));

mock.module("$app/navigation", () => ({
  goto: mock(() => Promise.resolve()),
}));

mock.module("@tauri-apps/api/core", () => ({
  invoke: mock(() => Promise.resolve()),
}));

// Mock window if it doesn't exist (Bun environment)
if (typeof window === 'undefined') {
  (globalThis as any).window = globalThis;
}
