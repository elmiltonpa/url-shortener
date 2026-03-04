import { ref } from "vue";

const SUBMIT_COOLDOWN_MS = 1000;

export function useFormSubmit() {
  const isLoading = ref(false);
  const errorMessage = ref("");
  let lastSubmitTime = 0;

  async function handleSubmit(submitFn: () => Promise<void>): Promise<void> {
    const now = Date.now();
    if (now - lastSubmitTime < SUBMIT_COOLDOWN_MS) return;
    if (isLoading.value) return;

    lastSubmitTime = now;
    isLoading.value = true;
    errorMessage.value = "";

    try {
      await submitFn();
    } catch (e) {
      if (!errorMessage.value) {
        errorMessage.value = "An unexpected error occurred";
      }
    } finally {
      isLoading.value = false;
    }
  }

  return {
    isLoading,
    errorMessage,
    handleSubmit,
  };
}
