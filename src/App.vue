<script setup>
import { ref } from "vue";
import { useRoute } from 'vue-router'
import { invoke } from "@tauri-apps/api/core";

// navigation
const navItems = ref([
  { 
    name: "Tasks", 
    link: "/", 
    description: "Manage running processes"
  },
  { 
    name: "Users", 
    link: "/users", 
    description: "user overview"
  }
]);

const route = useRoute();
const isMenuOpen = ref(false);

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value;
};

const closeMenu = () => {
  isMenuOpen.value = false;
};
</script>

<template>
  <div class="fixed select-none top-0 z-50 w-full p-2 bg-second text-emerald-400 text-lg font-bold flex justify-between items-center gap-2">
    <div class="flex items-center gap-2">
      
      <img src="./assets/images/logo/logo-transparent.png" class="w-12 p-2" alt="Logo">
      <h1>Inal Xan /</h1>
      
      <img src="./assets/images/icon/goat.png" class="w-12 p-2" alt="Logo">
    </div>
    
    <!-- Mobile menu button -->
    <button 
      @click="toggleMenu"
      class="md:hidden p-2 rounded-md text-emerald-400 hover:bg-gray-800 focus:outline-none"
      aria-label="Toggle menu"
    >
      <svg 
        xmlns="http://www.w3.org/2000/svg" 
        class="h-6 w-6" 
        fill="none" 
        viewBox="0 0 24 24" 
        stroke="currentColor"
      >
        <path 
          stroke-linecap="round" 
          stroke-linejoin="round" 
          stroke-width="2" 
          :d="isMenuOpen ? 'M6 18L18 6M6 6l12 12' : 'M4 6h16M4 12h16M4 18h16'" 
        />
      </svg>
    </button>
  </div>

  <!-- Mobile menu overlay -->
  <div 
    v-if="isMenuOpen"
    @click="closeMenu"
    class="fixed inset-0 z-40 bg-black bg-opacity-50 md:hidden"
  ></div>

  <div class="my-16 w-full bg-gray-900 text-gray-100 shadow-lg">
    <nav class="max-w-7xl mx-auto px-4">
      <!-- Desktop Navigation -->
      <ul class="hidden md:flex items-stretch">
        <li 
          v-for="(navItem, index) in navItems" 
          :key="index" 
          class="flex-1 relative group"
          :title="navItem.description"
        >
          <router-link 
            :to="navItem.link" 
            class="flex flex-col items-center px-4 py-4 text-sm font-medium transition-all duration-200 hover:bg-gray-800"
            :class="{
              'text-emerald-400 bg-gray-800': route.path === navItem.link,
              'text-gray-300 hover:text-emerald-300': route.path !== navItem.link
            }"
            @click="closeMenu"
          >
            <span>{{ navItem.name }}</span>
            <div 
              class="absolute bottom-0 left-1/2 transform -translate-x-1/2 h-0.5 bg-emerald-400 transition-all duration-300 w-0"
              :class="{
                'w-full': route.path === navItem.link,
                'group-hover:w-3/4': route.path !== navItem.link
              }"
            ></div>
          </router-link>
        </li>
      </ul>
      
      <!-- Mobile Navigation -->
      <ul 
        v-if="isMenuOpen"
        class="md:hidden absolute z-50 w-full left-0 bg-gray-900 shadow-xl"
        :class="{ 'block': isMenuOpen, 'hidden': !isMenuOpen }"
      >
        <li 
          v-for="(navItem, index) in navItems" 
          :key="index" 
          class="border-b border-gray-800"
          :title="navItem.description"
        >
          <router-link 
            :to="navItem.link" 
            class="block px-6 py-4 text-sm font-medium transition-colors duration-200"
            :class="{
              'text-emerald-400 bg-gray-800': route.path === navItem.link,
              'text-gray-300 hover:text-emerald-300 hover:bg-gray-800': route.path !== navItem.link
            }"
            @click="closeMenu"
          >
            <div class="flex items-center">
              <span>{{ navItem.name }}</span>
              <span class="ml-2 text-xs text-gray-500">{{ navItem.description }}</span>
            </div>
          </router-link>
        </li>
      </ul>
    </nav>
  </div>
  
  <div id="container" class="bg-gray-100 min-h-screen">
    <router-view class="max-w-7xl mx-auto p-6"/>
  </div>
</template>

<style>
#container {
  background: radial-gradient(circle at center, #f8fafc 0%, #e2e8f0 100%);
}

/* Smooth transitions for mobile menu */
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>