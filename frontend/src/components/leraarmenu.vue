<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="menu-item" to="/leraar">Home</router-link>
        <router-link tag="li" id="menu-item" to="/leraar/klassen">Klassen</router-link> 
        <router-link tag="li" id="menu-item" to="/leraar/profiel">Profiel</router-link> 
        <router-link tag="li" id="menu-item" to="/" v-on:click="this.$cookie.removeCookie('teacher_auth')" v-if="this.$cookie.isCookieAvailable('teacher_auth')">Uitloggen</router-link> 
        <router-link tag="li" id="menu-item" to="/leraar/login" v-else>Login</router-link> 
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
