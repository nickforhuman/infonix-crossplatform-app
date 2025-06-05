<template>
  <div class="w-full min-h-screen bg-gray-50 p-4 md:p-6">
    <div class="max-w-6xl mx-auto bg-white rounded-xl shadow-md overflow-hidden">
      <div class="p-4 bg-primary">
        <h1 class="text-xl md:text-2xl font-bold text-white">Task Manager</h1>
      </div>

      <!-- Mobile Cards View -->
      <div class="md:hidden">
        <div v-for="task in tasks" :key="task.id" class="p-4 border-b border-gray-200">
          <div class="flex justify-between items-start">
            <div class="flex items-center">
              <div class="w-3 h-3 rounded-full bg-blue-500 mr-3"></div>
              <span class="font-medium">{{ task.name }}</span>
            </div>
            <span class="px-2 py-1 rounded-full text-xs font-medium"
              :class="{
                'bg-green-100 text-green-800': task.status === 'Running',
                'bg-gray-100 text-gray-800': task.status === 'Idle',
                'bg-red-100 text-red-800': task.status === 'Stopped'
              }">
              {{ task.status }}
            </span>
          </div>
          
          <div class="mt-3 space-y-2">
            <div>
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>RAM Usage</span>
                <span>{{ task.ram.toFixed(1) }} MB</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="h-2 rounded-full" 
                  :class="{
                    'bg-green-500': task.ram < 50,
                    'bg-yellow-500': task.ram >= 50 && task.ram < 80,
                    'bg-red-500': task.ram >= 80
                  }" 
                  :style="{ width: Math.min(task.ram, 100) + '%' }">
                </div>
              </div>
            </div>
            
            <div>
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>CPU Usage</span>
                <span>{{ task.cpu.toFixed(1) }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="h-2 rounded-full" 
                  :class="{
                    'bg-green-500': task.cpu < 50,
                    'bg-yellow-500': task.cpu >= 50 && task.cpu < 80,
                    'bg-red-500': task.cpu >= 80
                  }" 
                  :style="{ width: task.cpu + '%' }">
                </div>
              </div>
            </div>
            
            <div>
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>Disk Usage</span>
                <span>{{ task.disk.toFixed(1) }} MB</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="h-2 rounded-full" 
                  :class="{
                    'bg-green-500': task.disk < 50,
                    'bg-yellow-500': task.disk >= 50 && task.disk < 80,
                    'bg-red-500': task.disk >= 80
                  }" 
                  :style="{ width: Math.min(task.disk, 100) + '%' }">
                </div>
              </div>
            </div>
          </div>
          
          <div class="mt-3 flex space-x-2">
            <button 
              @click="endTask(task.id)"
              class="flex-1 px-3 py-1.5 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors text-sm">
              End Task
            </button>
            <button 
              v-if="task.status !== 'Running'"
              @click="startTask(task.id)"
              class="flex-1 px-3 cursor-pointer py-1.5 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors text-sm">
              Start
            </button>
          </div>
        </div>
      </div>

      <!-- Desktop Table View -->
      <div class="hidden md:block overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-100">
            <tr>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Task Name</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">RAM Usage</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">CPU Usage</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Disk Usage</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Status</th>
              <th class="py-3 px-4 text-left font-semibold text-gray-700">Actions</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr v-for="task in tasks" :key="task.id" class="hover:bg-gray-50 transition-colors">
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <div class="w-3 h-3 rounded-full bg-blue-500 mr-3"></div>
                  <span class="font-medium">{{ task.name }}</span>
                </div>
              </td>
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <span class="mr-2">{{ task.ram.toFixed(1) }} MB</span>
                  <div class="w-24 bg-gray-200 rounded-full h-2.5">
                    <div 
                      class="h-2.5 rounded-full transition-all duration-300" 
                      :class="{
                        'bg-green-500': task.ram < 50,
                        'bg-yellow-500': task.ram >= 50 && task.ram < 80,
                        'bg-red-500': task.ram >= 80
                      }" 
                      :style="{ width: Math.min(task.ram, 100) + '%' }">
                    </div>
                  </div>
                </div>
              </td>
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <span class="mr-2">{{ task.cpu.toFixed(1) }}%</span>
                  <div class="w-24 bg-gray-200 rounded-full h-2.5">
                    <div 
                      class="h-2.5 rounded-full transition-all duration-300" 
                      :class="{
                        'bg-green-500': task.cpu < 50,
                        'bg-yellow-500': task.cpu >= 50 && task.cpu < 80,
                        'bg-red-500': task.cpu >= 80
                      }" 
                      :style="{ width: task.cpu + '%' }">
                    </div>
                  </div>
                </div>
              </td>
              <td class="py-4 px-4">
                <div class="flex items-center">
                  <span class="mr-2">{{ task.disk.toFixed(1) }} MB</span>
                  <div class="w-24 bg-gray-200 rounded-full h-2.5">
                    <div 
                      class="h-2.5 rounded-full transition-all duration-300" 
                      :class="{
                        'bg-green-500': task.disk < 50,
                        'bg-yellow-500': task.disk >= 50 && task.disk < 80,
                        'bg-red-500': task.disk >= 80
                      }" 
                      :style="{ width: Math.min(task.disk, 100) + '%' }">
                    </div>
                  </div>
                </div>
              </td>
              <td class="py-4 px-4">
                <span class="px-3 py-1 rounded-full text-xs font-medium"
                  :class="{
                    'bg-green-100 text-green-800': task.status === 'Running',
                    'bg-gray-100 text-gray-800': task.status === 'Idle',
                    'bg-red-100 text-red-800': task.status === 'Stopped'
                  }">
                  {{ task.status }}
                </span>
              </td>
              <td class="py-4 px-4">
                <div class="flex space-x-2">
                  <button 
                    @click="endTask(task.id)"
                    class="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2">
                    End Task
                  </button>
                  <button 
                    v-if="task.status !== 'Running'"
                    @click="startTask(task.id)"
                    class="px-4 py-2 cursor-pointer bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2">
                    Start
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="p-4 bg-gray-50 border-t border-gray-200 flex justify-between items-center">
        <div class="text-sm text-gray-600">
          Showing <span class="font-medium">1</span> to <span class="font-medium">{{ tasks.length }}</span> of <span class="font-medium">{{ tasks.length }}</span> tasks
        </div>
        <div class="flex space-x-2">
          <button 
            @click="refreshTasks"
            class="px-3 py-1 border rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50">
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

const tasks = ref([])

async function refreshTasks() {
  try {
    tasks.value = await invoke("get_system_tasks")
  } catch (error) {
    console.error("Failed to fetch tasks:", error)
    tasks.value = []
  }
}

async function startTask(pid) {
  try {
    await invoke("start_process", { pid })
    await refreshTasks()
  } catch (error) {
    console.error("Failed to start task:", error)
  }
}

async function endTask(pid) {
  try {
    await invoke("kill_process", { pid })
    await refreshTasks()
  } catch (error) {
    console.error("Failed to end task:", error)
  }
}

onMounted(() => {
  refreshTasks()
  setInterval(refreshTasks, 5000)
})
</script>