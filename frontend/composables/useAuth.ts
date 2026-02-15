export const useAuth = () => {
  const user = useState('auth_user', () => null)
  const token = useState('auth_token', () => null)
  const config = useRuntimeConfig()

  const login = async (email, password) => {
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/login`, {
        method: 'POST',
        body: { email, password }
      })

      if (response.status === 'success') {
        user.value = response.data.user
        token.value = response.data.token

        // In a real app, you'd also set a cookie here
        return { success: true }
      }
      return { success: false, message: response.message || 'Login failed' }
    } catch (err) {
      return { success: false, message: err.data?.message || 'An error occurred during login' }
    }
  }

  const register = async (userData) => {
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/register`, {
        method: 'POST',
        body: userData
      })
      
      if (response.status === 'success') {
        user.value = response.data.user
        token.value = response.data.token
        return { success: true }
      }
      return { success: false, message: response.message || 'Registration failed' }
    } catch (err) {
      return { success: false, message: err.data?.message || 'An error occurred during registration' }
    }
  }

  const logout = () => {
    user.value = null
    token.value = null
  }

  return {
    user,
    token,
    login,
    register,
    logout,
    isLoggedIn: computed(() => !!token.value)
  }
}
