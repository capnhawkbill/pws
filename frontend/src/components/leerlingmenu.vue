<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="button" to="/leerling">Home</router-link>
        <router-link tag="li" id="button" to="/leerling/rang">Rang</router-link> 
        <router-link tag="li" id="button" to="/leerling/klas">Klas</router-link> 
        <router-link tag="li" id="button" to="/leerling/winkel">Winkel</router-link> 
        <router-link tag="li" id="button" to="/leerling/profiel">Profiel</router-link> 
        <router-link tag="li" id="button" to="/leerling/login">Login</router-link> 
      </ol>
    </nav>
  </header>
  <main>
    <router-view/>
  </main>
</template>

<script>
  export default {
    name: 'leerlingmenu'
  }
</script>
