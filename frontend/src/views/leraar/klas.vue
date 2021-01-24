<template>
  <section v-if="errored">
    <p>Error met API request. Weet je zeker dat deze klas bestaat?</p>
  </section>

  <section v-else>
    <div v-if="loading">Loading...</div>
  
    <div> 
      {{ klasinfo }}
      <table class="class">
        <thead>
            <tr>
                <th v-for="(column, index) in this.columns" :key="index"> {{column}}</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="(item, index) in this.klasinfo" :key="index">
                <td v-for="(column, indexColumn) in this.columns" :key="indexColumn">{{item[column]}}</td>
            </tr>
        </tbody>
      </table>
      Nodig leerlingen uit met deze link:
      <button class='left' v-on:click="copyShareLink">Kopieer link</button>
    </div>
  </section>
</template>

<script>
export default {
  data () {
    return {
      klasinfo: null,
      columns: ['Naam', 'Punten'],
      loading: true,
      errored: false
    }
  },
  methods: {
    copyShareLink() {
      var dummy = document.createElement("textarea");
      document.body.appendChild(dummy);
      dummy.value = window.location.origin + '/#/leerling/klassen/join/' + this.$route.params.id;
      dummy.select();
      document.execCommand("copy");
      document.body.removeChild(dummy);
}
  },
  mounted () {
    this.axios
      .get('/api/class/leaderboard?class=' + this.$route.params.id, {'headers': {'Authorization': document.cookie}})
      .then(response => (this.klasinfo = response.data))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
      .finally(() => this.loading = false)
  }
}

</script>
