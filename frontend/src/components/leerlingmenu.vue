<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="menu-item" to="/leerling">Home</router-link>
        <router-link tag="li" id="menu-item" to="/leerling/klassen">Klassen</router-link> 
        <router-link tag="li" id="menu-item" to="/leerling/profiel">Profiel</router-link> 
        <router-link tag="li" id="menu-item" to="/" v-on:click="this.$cookie.removeCookie('student_auth')" v-if="this.$cookie.isCookieAvailable('student_auth')">Uitloggen</router-link> 
        <router-link tag="li" v-else id="menu-item" to="/leerling/login">Login</router-link> 
      </ol>
    </nav>
  </header>
  <main>
    <router-view/>
  </main>
</template>

<script>
  export default {
    name: 'leerlingmenu',
    methods: {
      uitloggen() {
        this.$cookie.removeCookie('student_auth')
        console.log('uitgelogd')
      }
    }
  }
</script>
