export const useAuth = () => {
  const user = useState('auth_user', () => null)
  const token = useState('auth_token', () => null)
  const cookieToken = useCookie('auth_token', { maxAge: 60 * 60 * 24 })
  const cookieUser = useCookie('auth_user', { maxAge: 60 * 60 * 24 })

  // Initialize from cookies
  if (cookieToken.value && !token.value) {
    token.value = cookieToken.value
  }
  if (cookieUser.value && !user.value) {
    user.value = cookieUser.value
  }

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
        
        cookieToken.value = response.data.token
        cookieUser.value = response.data.user

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

        cookieToken.value = response.data.token
        cookieUser.value = response.data.user

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
    cookieToken.value = null
    cookieUser.value = null
  }

  const updateProfile = async (userData) => {
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/profile`, {
        method: 'PUT',
        headers: {
          'Authorization': token.value
        },
        body: userData
      })
      if (response.status === 'success') {
        user.value = response.data.user
        cookieUser.value = response.data.user
        return { success: true, message: response.message }
      }
      return { success: false, message: response.message || 'Update failed' }
    } catch (err) {
      return { success: false, message: err.data?.message || 'An error occurred during update' }
    }
  }

  const deleteAccount = async () => {
    try {
      const response = await $fetch(`${config.public.apiBase}/auth/delete-account`, {
        method: 'DELETE',
        headers: {
          'Authorization': token.value
        }
      })
      if (response.status === 'success') {
        logout()
        return { success: true }
      }
      return { success: false, message: response.message || 'Deletion failed' }
    } catch (err) {
      return { success: false, message: err.data?.message || 'An error occurred during deletion' }
    }
  }

  return {
    user,
    token,
    login,
    register,
    logout,
    updateProfile,
    deleteAccount,
    isLoggedIn: computed(() => !!token.value)
  }
}
