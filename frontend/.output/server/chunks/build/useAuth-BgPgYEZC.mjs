import { computed, toRef, isRef } from 'vue';
import { a as useNuxtApp, b as useRuntimeConfig } from './server.mjs';

const useStateKeyPrefix = "$s";
function useState(...args) {
  const autoKey = typeof args[args.length - 1] === "string" ? args.pop() : void 0;
  if (typeof args[0] !== "string") {
    args.unshift(autoKey);
  }
  const [_key, init] = args;
  if (!_key || typeof _key !== "string") {
    throw new TypeError("[nuxt] [useState] key must be a string: " + _key);
  }
  if (init !== void 0 && typeof init !== "function") {
    throw new Error("[nuxt] [useState] init must be a function: " + init);
  }
  const key = useStateKeyPrefix + _key;
  const nuxtApp = useNuxtApp();
  const state = toRef(nuxtApp.payload.state, key);
  if (state.value === void 0 && init) {
    const initialValue = init();
    if (isRef(initialValue)) {
      nuxtApp.payload.state[key] = initialValue;
      return initialValue;
    }
    state.value = initialValue;
  }
  return state;
}
const useAuth = () => {
  const user = useState("auth_user", () => null);
  const token = useState("auth_token", () => null);
  const config = useRuntimeConfig();
  const login = async (email, password) => {
    var _a;
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/login`, {
        method: "POST",
        body: { email, password }
      });
      if (response.status === "success") {
        user.value = response.data.user;
        token.value = response.data.token;
        return { success: true };
      }
      return { success: false, message: response.message || "Login failed" };
    } catch (err) {
      return { success: false, message: ((_a = err.data) == null ? void 0 : _a.message) || "An error occurred during login" };
    }
  };
  const register = async (userData) => {
    var _a;
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/register`, {
        method: "POST",
        body: userData
      });
      if (response.status === "success") {
        user.value = response.data.user;
        token.value = response.data.token;
        return { success: true };
      }
      return { success: false, message: response.message || "Registration failed" };
    } catch (err) {
      return { success: false, message: ((_a = err.data) == null ? void 0 : _a.message) || "An error occurred during registration" };
    }
  };
  const logout = () => {
    user.value = null;
    token.value = null;
  };
  const updateProfile = async (userData) => {
    var _a;
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/profile`, {
        method: "PUT",
        headers: {
          "Authorization": token.value
        },
        body: userData
      });
      if (response.status === "success") {
        user.value = response.data.user;
        return { success: true, message: response.message };
      }
      return { success: false, message: response.message || "Update failed" };
    } catch (err) {
      return { success: false, message: ((_a = err.data) == null ? void 0 : _a.message) || "An error occurred during update" };
    }
  };
  const deleteAccount = async () => {
    var _a;
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/delete-account`, {
        method: "DELETE",
        headers: {
          "Authorization": token.value
        }
      });
      if (response.status === "success") {
        logout();
        return { success: true };
      }
      return { success: false, message: response.message || "Deletion failed" };
    } catch (err) {
      return { success: false, message: ((_a = err.data) == null ? void 0 : _a.message) || "An error occurred during deletion" };
    }
  };
  return {
    user,
    token,
    login,
    register,
    logout,
    updateProfile,
    deleteAccount,
    isLoggedIn: computed(() => !!token.value)
  };
};

export { useAuth as u };
//# sourceMappingURL=useAuth-BgPgYEZC.mjs.map
