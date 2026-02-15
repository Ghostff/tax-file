<template>
  <div class="max-w-4xl mx-auto py-8 px-4 sm:px-6 lg:px-8">
    <div v-if="isLoggedIn">
      <div class="md:grid md:grid-cols-3 md:gap-6">
        <div class="md:col-span-1">
          <div class="px-4 sm:px-0">
            <h3 class="text-lg font-medium leading-6 text-gray-900">Profile</h3>
            <p class="mt-1 text-sm text-gray-600">Update your account information.</p>
          </div>
        </div>
        <div class="mt-5 md:mt-0 md:col-span-2">
          <form @submit.prevent="handleUpdateProfile">
            <div class="shadow sm:rounded-md sm:overflow-hidden">
              <div class="px-4 py-5 bg-white space-y-6 sm:p-6">
                <div class="grid grid-cols-6 gap-6">
                  <div class="col-span-6 sm:col-span-3">
                    <label class="block text-sm font-medium text-gray-700">First name</label>
                    <input v-model="profileForm.first_name" type="text" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                  </div>
                  <div class="col-span-6 sm:col-span-3">
                    <label class="block text-sm font-medium text-gray-700">Last name</label>
                    <input v-model="profileForm.last_name" type="text" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                  </div>
                  <div class="col-span-6">
                    <label class="block text-sm font-medium text-gray-700">Email address</label>
                    <input v-model="profileForm.email" type="email" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                  </div>
                  <div class="col-span-6">
                    <label class="block text-sm font-medium text-gray-700">New Password (optional)</label>
                    <input v-model="profileForm.password" type="password" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                  </div>
                </div>
              </div>
              <div class="px-4 py-3 bg-gray-50 text-right sm:px-6 flex justify-between">
                <button @click="confirmDelete" type="button" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                  Delete Account
                </button>
                <button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  Save
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>

      <div class="hidden sm:block" aria-hidden="true">
        <div class="py-5">
          <div class="border-t border-gray-200"></div>
        </div>
      </div>

      <!-- Tax Filing Section -->
      <div class="md:grid md:grid-cols-3 md:gap-6 mt-10">
        <div class="md:col-span-1">
          <div class="px-4 sm:px-0">
            <h3 class="text-lg font-medium leading-6 text-gray-900">Tax Filing</h3>
            <p class="mt-1 text-sm text-gray-600">File your taxes for the current year or view history.</p>
          </div>
        </div>
        <div class="mt-5 md:mt-0 md:col-span-2">
          <div class="bg-white shadow sm:rounded-lg p-6">
            <div class="flex items-center justify-between mb-6">
              <h4 class="text-lg font-medium">File Tax for 2024</h4>
              <button @click="showUpload = !showUpload" class="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700">
                {{ showUpload ? 'Cancel' : 'Start Filing' }}
              </button>
            </div>

            <div v-if="showUpload" class="space-y-6">
              <div class="border-2 border-dashed border-gray-300 rounded-lg p-12 text-center">
                <input type="file" ref="fileInput" @change="handleFileUpload" class="hidden">
                <div v-if="!uploading" class="space-y-1">
                  <p class="text-gray-600">Upload W2 or 1099</p>
                  <button @click="$refs.fileInput.click()" class="text-indigo-600 font-medium hover:text-indigo-500">
                    Click to select a file
                  </button>
                </div>
                <div v-else class="text-indigo-600">Uploading and extracting data...</div>
              </div>
              
              <div class="relative">
                <div class="absolute inset-0 flex items-center" aria-hidden="true">
                  <div class="w-full border-t border-gray-300"></div>
                </div>
                <div class="relative flex justify-center text-sm">
                  <span class="px-2 bg-white text-gray-500">Or enter manually</span>
                </div>
              </div>

              <div class="space-y-4">
                <div v-for="(item, index) in manualItems" :key="index" class="p-4 border rounded-md relative">
                  <button @click="manualItems.splice(index, 1)" class="absolute top-2 right-2 text-red-500">×</button>
                  <div class="grid grid-cols-2 gap-4">
                    <input v-model="item.label" placeholder="Description (e.g. Interest)" class="border rounded px-2 py-1">
                    <input v-model="item.amount" type="number" placeholder="Amount" class="border rounded px-2 py-1">
                  </div>
                </div>
                <button @click="manualItems.push({label: '', amount: 0})" class="text-sm text-indigo-600 font-medium">+ Add item</button>
                <div class="text-right">
                  <button @click="saveManual" class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">Save Manual Info</button>
                </div>
              </div>
            </div>

            <div v-if="taxData.length > 0" class="mt-8">
              <h5 class="font-medium mb-3">Extracted Tax Summary</h5>
              <div class="space-y-4">
                <div v-for="data in taxData" :key="data.id" class="border rounded-md p-4 bg-gray-50">
                  <h6 class="font-bold text-indigo-700">Year: {{ data.year }}</h6>
                  <div class="mt-2 text-sm">
                    <div v-if="data.data.documents && data.data.documents.length > 0">
                      <div v-for="(doc, idx) in data.data.documents" :key="idx" class="mt-2 pb-2 border-b last:border-0">
                        <div class="font-medium text-gray-800">{{ doc.type }}:</div>
                        <!-- Support multiple records per document -->
                        <div v-if="doc.records && doc.records.length > 0" class="ml-2 space-y-1">
                          <div v-for="(record, rIdx) in doc.records" :key="rIdx" class="text-xs text-gray-600">
                            <span v-if="record.employer">Employer: {{ record.employer }}, Wages: ${{ record.wages }}</span>
                            <span v-else-if="record.payer">Payer: {{ record.payer }}, Income: ${{ record.income }}</span>
                            <span v-else-if="record.error" class="text-red-400 italic">{{ record.error }}</span>
                          </div>
                        </div>
                        <!-- Fallback for old format -->
                        <div v-else-if="doc.extracted" class="ml-2 text-xs text-gray-600">
                          <span v-if="doc.extracted.employer">Employer: {{ doc.extracted.employer }}, Wages: ${{ doc.extracted.wages }}</span>
                          <span v-else-if="doc.extracted.payer">Payer: {{ doc.extracted.payer }}, Income: ${{ doc.extracted.interest_income || doc.extracted.income }}</span>
                        </div>
                      </div>
                    </div>
                    <div v-if="data.data.manual && data.data.manual.length > 0" class="mt-2 pt-2 border-t">
                      <p class="italic text-gray-500">Manual Entries:</p>
                      <div v-for="(m, idx) in data.data.manual" :key="idx" class="ml-2">
                        - {{ m.label }}: ${{ m.amount }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="documents.length > 0" class="mt-8">
              <h5 class="font-medium mb-3">Previous Uploads</h5>
              <ul class="divide-y divide-gray-200">
                <li v-for="doc in documents" :key="doc.id" class="py-3 flex justify-between items-center">
                  <div>
                    <span class="font-medium">{{ doc.year }} {{ doc.document_type }}</span>
                    <span class="text-sm text-gray-500 ml-2">{{ doc.file_name }}</span>
                  </div>
                  <button @click="downloadDoc(doc)" class="text-indigo-600 hover:text-indigo-900 text-sm font-medium">Download</button>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <!-- AI Helper Section -->
      <div class="fixed bottom-4 right-4 w-80 md:w-96">
        <div v-if="showAI" class="bg-white rounded-lg shadow-xl border flex flex-col h-96">
          <div class="p-4 bg-indigo-600 text-white rounded-t-lg flex justify-between items-center">
            <span class="font-medium">Tax AI Helper</span>
            <button @click="showAI = false" class="text-white hover:text-gray-200">×</button>
          </div>
          <div class="flex-1 p-4 overflow-y-auto space-y-4" ref="chatContainer">
            <div v-for="(msg, i) in chatHistory" :key="i" :class="msg.role === 'user' ? 'text-right' : 'text-left'">
              <span :class="msg.role === 'user' ? 'bg-indigo-100' : 'bg-gray-100'" class="inline-block px-3 py-2 rounded-lg text-sm max-w-[80%]">
                {{ msg.content }}
              </span>
            </div>
          </div>
          <div class="p-3 border-t">
            <form @submit.prevent="askAI" class="flex gap-2">
              <input v-model="aiQuestion" placeholder="Ask a tax question..." class="flex-1 border rounded-md px-3 py-1 text-sm">
              <button type="submit" class="bg-indigo-600 text-white px-3 py-1 rounded-md text-sm">Ask</button>
            </form>
          </div>
        </div>
        <button v-else @click="showAI = true" class="bg-indigo-600 text-white p-4 rounded-full shadow-lg hover:bg-indigo-700">
          <span class="sr-only">AI Helper</span>
          🤖
        </button>
      </div>
    </div>
    
    <div v-else class="text-center py-20">
      <h2 class="text-2xl font-bold">Welcome to Tax File App</h2>
      <p class="mt-4">Please <NuxtLink to="/login" class="text-indigo-600">Login</NuxtLink> or <NuxtLink to="/register" class="text-indigo-600">Register</NuxtLink> to continue.</p>
    </div>
  </div>
</template>

<script setup>
const { user, token, isLoggedIn, updateProfile, deleteAccount, logout } = useAuth()
const config = useRuntimeConfig()

const profileForm = ref({
  first_name: user.value?.first_name || '',
  last_name: user.value?.last_name || '',
  email: user.value?.email || '',
  password: ''
})

const handleUpdateProfile = async () => {
  const res = await updateProfile(profileForm.value)
  if (res.success) alert('Profile updated!')
  else alert(res.message)
}

const confirmDelete = async () => {
  if (confirm('Are you sure you want to delete your account? This action is permanent.')) {
    const res = await deleteAccount()
    if (res.success) {
      alert('Account deleted.')
      navigateTo('/')
    } else alert(res.message)
  }
}

// Tax Filing Logic
const showUpload = ref(false)
const uploading = ref(false)
const documents = ref([])
const manualItems = ref([{ label: '', amount: 0 }])
const taxData = ref([])

const fetchDocs = async () => {
  if (!isLoggedIn.value) return
  try {
    const res = await $fetch(`${config.public.apiBase}/tax/documents`, {
      headers: { 'Authorization': token.value }
    })
    if (res.status === 'success') documents.value = res.data.documents
  } catch (err) {}
}

const fetchTaxData = async () => {
  if (!isLoggedIn.value) return
  try {
    const res = await $fetch(`${config.public.apiBase}/tax/data`, {
      headers: { 'Authorization': token.value }
    })
    if (res.status === 'success') taxData.value = res.data.tax_data
  } catch (err) {}
}

const handleFileUpload = async (event) => {
  const file = event.target.files[0]
  if (!file) return

  uploading.value = true
  const formData = new FormData()
  formData.append('file', file)
  formData.append('year', '2024')
  formData.append('document_type', file.name.includes('1099') ? '1099' : 'W2')

  try {
    const res = await $fetch(`${config.public.apiBase}/tax/upload`, {
      method: 'POST',
      headers: { 'Authorization': token.value },
      body: formData
    })
    if (res.status === 'success') {
      alert('Document uploaded and data extracted!')
      console.log('Extracted Data:', res.data.extracted_data)
      fetchDocs()
      fetchTaxData()
    } else {
      alert(`Upload failed: ${res.message || 'Unknown error'}`)
    }
  } catch (err) {
    console.error('Upload error:', err)
    alert(`Upload failed: ${err.data?.message || err.message || 'An error occurred'}`)
  } finally {
    uploading.value = false
    showUpload.value = false
  }
}

const saveManual = async () => {
  try {
    const res = await $fetch(`${config.public.apiBase}/tax/data`, {
      method: 'POST',
      headers: { 'Authorization': token.value },
      body: {
        year: 2024,
        data: { manual: manualItems.value }
      }
    })
    if (res.status === 'success') alert('Manual data saved!')
  } catch (err) {
    alert('Save failed')
  }
}

const downloadDoc = async (doc) => {
  const url = `${config.public.apiBase}/tax/download/${doc.id}`
  const a = document.createElement('a')
  a.href = url
  // Add token for authentication if the backend supports it via query param or just use fetch
  // For simplicity, let's assume direct link works if we add token (or backend handles it)
  // Since it's a file, we might need to fetch it as a blob
  try {
    const response = await fetch(url, {
      headers: { 'Authorization': token.value }
    })
    const blob = await response.blob()
    const blobUrl = window.URL.createObjectURL(blob)
    a.href = blobUrl
    a.download = doc.file_name
    a.click()
  } catch (err) {
    alert('Download failed')
  }
}

// AI Helper Logic
const showAI = ref(false)
const aiQuestion = ref('')
const chatHistory = ref([
  { role: 'assistant', content: 'Hello! I am your tax helper. I have access to your tax records. How can I help you today?' }
])
const chatContainer = ref(null)

const askAI = async () => {
  if (!aiQuestion.value.trim()) return
  
  const q = aiQuestion.value
  chatHistory.value.push({ role: 'user', content: q })
  aiQuestion.value = ''
  
  nextTick(() => {
    if (chatContainer.value) chatContainer.value.scrollTop = chatContainer.value.scrollHeight
  })

  try {
    const res = await $fetch(`${config.public.apiBase}/tax/ai-helper`, {
      method: 'POST',
      headers: { 'Authorization': token.value },
      body: { question: q }
    })
    if (res.status === 'success') {
      chatHistory.value.push({ role: 'assistant', content: res.data.answer })
    }
  } catch (err) {
    chatHistory.value.push({ role: 'assistant', content: 'Sorry, I am having trouble connecting right now.' })
  }

  nextTick(() => {
    if (chatContainer.value) chatContainer.value.scrollTop = chatContainer.value.scrollHeight
  })
}

const router = useRouter()

onMounted(() => {
  if (!isLoggedIn.value) {
    router.push('/login')
  } else {
    fetchDocs()
    fetchTaxData()
  }
})
</script>
