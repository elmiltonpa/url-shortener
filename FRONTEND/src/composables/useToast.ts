import { ref } from "vue";

export type ToastType = "success" | "error" | "info";

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
}

const toasts = ref<Toast[]>([]);
let nextId = 0;

export function useToast() {
  const addToast = (
    message: string,
    type: ToastType = "info",
    duration = 3500,
  ) => {
    const id = nextId++;
    toasts.value.push({ id, message, type });
    setTimeout(() => {
      dismiss(id);
    }, duration);
  };

  const success = (message: string) => addToast(message, "success", 3000);
  const error = (message: string) => addToast(message, "error", 4500);
  const info = (message: string) => addToast(message, "info", 3000);

  const dismiss = (id: number) => {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  };

  return {
    toasts,
    success,
    error,
    info,
    dismiss,
  };
}
