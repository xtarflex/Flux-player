
import { mock } from "bun:test";

const writable = (initialValue: any) => {
  let value = initialValue;
  const subscribers = new Set<(v: any) => void>();
  return {
    subscribe: (fn: (v: any) => void) => {
      subscribers.add(fn);
      fn(value);
      return () => subscribers.delete(fn);
    },
    set: (v: any) => {
      value = v;
      subscribers.forEach(fn => fn(value));
    },
    update: (fn: (v: any) => any) => {
      value = fn(value);
      subscribers.forEach(fn => fn(value));
    }
  };
};

const getStoreValue = (store: any) => {
  let value;
  store.subscribe((v: any) => value = v)();
  return value;
};

// Mock svelte/store
mock.module("svelte/store", () => ({
  writable,
  get: getStoreValue
}));

// Mock @tauri-apps/api/core
mock.module("@tauri-apps/api/core", () => ({
  invoke: () => Promise.resolve(),
  convertFileSrc: (path: string) => `asset://${path}`
}));

// Mock @tauri-apps/api/event
mock.module("@tauri-apps/api/event", () => ({
  listen: () => Promise.resolve(() => {})
}));

// Mock ./settings
mock.module("./settings", () => ({
  settings: writable({ watchedThreshold: 90 })
}));

export { writable, getStoreValue as get };
