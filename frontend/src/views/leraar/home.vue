<template>
  <button class='left' v-on:click="create_class">+ Maak klas aan</button>
  <section v-if="errored">
    <p>Error with API request.</p>
  </section>

  <section v-else>
    <div v-if="loading">Loading...</div>
  
    <div id="class">
      {{ klasinfo }}
    </div>
  </section>
</template>

<script>
export default {
  name: 'classes',
  data () {
    return {
      klasinfo: null,
      loading: true,
      errored: false
    }
  },
  mounted () {
    this.axios
      .get('/api/teacher/info', {"headers": {"Authorization": document.cookie}})
      .then(response => (this.klassen = response.data.classes))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
      .finally(() => this.loading = false)
    }
  }
</script>
