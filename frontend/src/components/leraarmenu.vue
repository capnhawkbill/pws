<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="button" to="/leraar">Home</router-link>
        <router-link tag="li" id="button" to="/leraar/klassen">Klassen</router-link> 
        <router-link tag="li" id="button" to="/leraar/profiel">Profiel</router-link> 
        <router-link tag="li" id="button" to="/leraar/login">Login</router-link> 
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
