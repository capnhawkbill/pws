<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="menu-item" to="/leraar">Home</router-link>
        <router-link tag="li" id="menu-item" to="/leraar/klassen">Klassen</router-link> 
        <router-link tag="li" id="menu-item" to="/leraar/profiel">Profiel</router-link> 
        <router-link tag="li" id="menu-item" to="/leraar/login">Login</router-link> 
      </ol>
    </nav>
  </header>
  <main>
    <router-view />
  </main>
</template>

<script>
  export default {
    name: 'leraarmenu'
  }
</script>
