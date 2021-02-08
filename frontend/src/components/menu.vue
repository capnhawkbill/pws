<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="menu-item" :to="{name: 'home'}">Home</router-link>
        <router-link tag="li" id="menu-item" :to="{name: 'klassen'}">Klassen</router-link> 
        <router-link tag="li" id="menu-item" :to="{name: 'profiel'}">Profiel</router-link> 
      </ol>
    </nav>
  </header>
  <main>
    <router-view/>
  </main>
</template>

<script>
  export default {
    name: 'menu'
  }
</script>
