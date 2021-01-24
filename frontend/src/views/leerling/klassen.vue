<template>
  <button class='left' v-on:click="this.$router.push('/leraar/klassen/aanmaken')">+ Maak klas aan</button>
  <section v-if="errored">
    <p>Error with API request.</p>
  </section>

  <section v-else>
    <div v-if="loading">Loading...</div>
  
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
  data () {
    return {
      klasinfo: null,
      loading: true,
      errored: false
    }
  },
  mounted () {
    this.axios
      .get('https://api.coindesk.com/v1/bpi/currentprice.json')
      .then(response => (this.klasinfo = response.data.bpi))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
      .finally(() => this.loading = false)
  }
}

</script>
