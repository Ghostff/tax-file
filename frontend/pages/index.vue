<template>
  <div class="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-4">
    <div class="max-w-md w-full text-center space-y-8">
      <div>
        <h1 class="text-4xl font-bold text-gray-900 tracking-tight">Tax File App</h1>
        <p class="mt-3 text-lg text-gray-600">Securely manage your tax documents and compliance.</p>
      </div>
      
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <NuxtLink 
          v-if="!isLoggedIn"
          to="/login" 
          class="flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 md:py-4 md:text-lg md:px-10"
        >
          Login
        </NuxtLink>
        <NuxtLink 
          v-if="!isLoggedIn"
          to="/register" 
          class="flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200 md:py-4 md:text-lg md:px-10"
        >
          Register
        </NuxtLink>
        <NuxtLink 
          v-if="isLoggedIn"
          to="/dashboard" 
          class="flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 md:py-4 md:text-lg md:px-10 col-span-2"
        >
          Go to Dashboard
        </NuxtLink>
      </div>
      
      <div class="mt-10">
        <div class="relative">
          <div class="absolute inset-0 flex items-center" aria-hidden="true">
            <div class="w-full border-t border-gray-300"></div>
          </div>
          <div class="relative flex justify-center text-sm">
            <span class="px-2 bg-gray-50 text-gray-500">Built with Nuxt 3 & Rust</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const { isLoggedIn } = useAuth()
const router = useRouter()

onMounted(() => {
  if (isLoggedIn.value) {
    router.push('/dashboard')
  }
})
</script>
