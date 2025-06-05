<template>
  <div class="w-full min-h-screen bg-gray-50 p-4 md:p-6">
    <div class="max-w-6xl mx-auto bg-white rounded-xl shadow-md overflow-hidden">
      <div class="p-4 bg-primary">
        <h1 class="text-xl md:text-2xl font-bold text-white">Active Users & Processes</h1>
      </div>

      <!-- Mobile Cards View -->
      <div class="md:hidden">
        <div v-for="user in users" :key="user.id" class="p-4 border-b border-gray-200">
          <div class="flex justify-between items-start">
            <div class="flex items-center">
              <div class="w-3 h-3 rounded-full bg-blue-500 mr-3"></div>
              <div>
                <span class="font-medium block">{{ user.username }}</span>
                <span class="font-medium block">{{ user.name }}</span>
                <span class="text-xs text-gray-500">PID: {{ user.id }}</span>
              </div>
            </div>
            <span class="px-2 py-1 rounded-full text-xs font-medium"
              :class="{
                'bg-green-100 text-green-800': user.status === 'Running',
                'bg-gray-100 text-gray-800': user.status === 'Idle',
                'bg-red-100 text-red-800': user.status === 'Stopped'
              }">
              {{ user.status }}
            </span>
          </div>
          
          <div class="mt-3 space-y-2">
            <div>
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>RAM Usage</span>
                <span>{{ user.ram.toFixed(1) }} MB</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="h-2 rounded-full" 
                  :class="{
                    'bg-green-500': user.ram < 50,
                    'bg-yellow-500': user.ram >= 50 && user.ram < 80,
                    'bg-red-500': user.ram >= 80
                  }" 
                  :style="{ width: Math.min(user.ram / 10, 100) + '%' }">
                </div>
              </div>
            </div>
            
            <div>
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>CPU Usage</span>
                <span>{{ user.cpu.toFixed(1) }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="h-2 rounded-full" 
                  :class="{
                    'bg-green-500': user.cpu < 50,
                    'bg-yellow-500': user.cpu >= 50 && user.cpu < 80,
                    'bg-red-500': user.cpu >= 80
                  }" 
                  :style="{ width: user.cpu + '%' }">
                </div>
              </div>
            </div>
            
            <!-- <div>
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>Disk Usage</span>
                <span>{{ user.disk.toFixed(1) }} MB</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="h-2 rounded-full" 
                  :class="{
                    'bg-green-500': user.disk < 50,
                    'bg-yellow-500': user.disk >= 50 && user.disk < 80,
                    'bg-red-500': user.disk >= 80
                  }" 
                  :style="{ width: Math.min(user.disk / 10, 100) + '%' }"> 
                </div>
              </div>
            </div> -->
          </div>
          
          <div class="mt-3 text-xs text-gray-500 space-y-1">
            <div class="flex justify-between">
              <span>UID/GID:</span>
              <span>{{ user.uid }}/{{ user.gid }}</span>
            </div>
            <div class="flex justify-between">
              <span>Group:</span>
              <span>{{ user.group }}</span>
            </div>
          </div>
          
          <div class="mt-3 flex space-x-2">
            <button 
              @click="killProcess(user.id)"
              class="flex-1 cursor-pointer px-3 py-1.5 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors text-sm">
              Kill Process
            </button>
          </div>
        </div>
      </div>

      <!-- Desktop Table View -->
      <div class="hidden md:block overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-100">
            <tr>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Username</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Process Name</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">PID</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">RAM Usage</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">CPU Usage</th>
              <!-- <th class="py-3 px-4 text-left font-semibold text-gray-700">Disk Usage</th> -->
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Status</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Actions</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50 transition-colors">
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <div class="w-3 h-3 rounded-full bg-blue-500 mr-3"></div>
                  <span class="font-medium">{{ user.username }}</span>
                </div>
              </td>
              
               <td class="py-4 px-4 text-sm text-gray-600">
                {{ user.name }}
              </td>
              <td class="py-4 px-4 text-sm text-gray-600">
                {{ user.id }}
              </td>
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <span class="mr-2">{{ user.ram.toFixed(1) }} MB</span>
                  <div class="w-24 bg-gray-200 rounded-full h-2.5">
                    <div 
                      class="h-2.5 rounded-full transition-all duration-300" 
                      :class="{
                        'bg-green-500': user.ram < 50,
                        'bg-yellow-500': user.ram >= 50 && user.ram < 80,
                        'bg-red-500': user.ram >= 80
                      }" 
                      :style="{ width: Math.min(user.ram / 10, 100) + '%' }">
                    </div>
                  </div>
                </div>
              </td>
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <span class="mr-2">{{ user.cpu.toFixed(1) }}%</span>
                  <div class="w-24 bg-gray-200 rounded-full h-2.5">
                    <div 
                      class="h-2.5 rounded-full transition-all duration-300" 
                      :class="{
                        'bg-green-500': user.cpu < 50,
                        'bg-yellow-500': user.cpu >= 50 && user.cpu < 80,
                        'bg-red-500': user.cpu >= 80
                      }" 
                      :style="{ width: user.cpu + '%' }">
                    </div>
                  </div>
                </div>
              </td>
              <!-- <td class="py-4 px-4">
                <div class="flex items-center">
                  <span class="mr-2">{{ user.disk.toFixed(1) }} MB</span>
                  <div class="w-24 bg-gray-200 rounded-full h-2.5">
                    <div 
                      class="h-2.5 rounded-full transition-all duration-300" 
                      :class="{
                        'bg-green-500': user.disk < 50,
                        'bg-yellow-500': user.disk >= 50 && user.disk < 80,
                        'bg-red-500': user.disk >= 80
                      }" 
                      :style="{ width: Math.min(user.disk / 10, 100) + '%' }"> 
                    </div>
                  </div>
                </div>
              </td> -->
              <td class="py-4 px-4">
                <span class="px-3 py-1 rounded-full text-xs font-medium"
                  :class="{
                    'bg-green-100 text-green-800': user.status === 'Running',
                    'bg-gray-100 text-gray-800': user.status === 'Idle',
                    'bg-red-100 text-red-800': user.status === 'Stopped'
                  }">
                  {{ user.status }}
                </span>
              </td>
              <td class="py-4 px-4">
                <div class="flex space-x-2">
                  <button 
                    @click="killProcess(user.id)"
                    class="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2">
                    Kill Process
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="p-4 bg-gray-50 border-t border-gray-200 flex justify-between items-center">
        <div class="text-sm text-gray-600">
          Showing <span class="font-medium">1</span> to <span class="font-medium">{{ users.length }}</span> of <span class="font-medium">{{ users.length }}</span> processes
        </div>
        <div class="flex space-x-2">
          <button 
            @click="refreshUsers"
            class="px-3 py-1 border cursor-pointer rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50">
            Refresh
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core"

const users = ref([])

async function refreshUsers() {
  try {
    users.value = await invoke("get_administration_user")
  } catch (error) {
    console.error("Failed to fetch users:", error)
    users.value = []
  }
}

async function killProcess(pid) {
  try {
    await invoke("kill_process", { pid })
    await refreshUsers()
  } catch (error) {
    console.error("Failed to kill process:", error)
  }
}

onMounted(() => {
  refreshUsers()
  setInterval(refreshUsers, 5000)
})
</script>