<template>
  <section v-if="errored">
    <p>Error met API request.</p>
  </section>

  <section v-else>
    <div v-if="loading">Laden...</div>
  
    <div id="class">
      {{ klasinfo }}
      <li 
        v-for="currency in klasinfo" 
        :key="currency"
      >
        {{ currency.description }}
      </li>
    </div>
  </section>
</template>

<script>
export default {
  name: 'class',
  data () {
    return {
      klasinfo: null,
      loading: true,
      errored: false
    }
  },
  mounted () {
    this.axios
      .get('/api/student/info')
      .then(response => (this.klasinfo = response.data.bpi))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
      .finally(() => this.loading = false)
  }
}

</script>
