<template>
  <div class="overflow-x-auto">
    <table class="table table-xs table-pin-rows table-pin-cols">
      <thead>
        <tr>
          <th></th>
          <td>White</td>
          <td>Black</td>
          <td>Result</td>
          <td>Opening</td>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="game in gameList" :key="game.id">
          <td>{{ game.id }}</td>
          <td>{{ game.white_player.name }}</td>
          <td>{{ game.black_player.name }}</td>
          <td>{{ game.result }}</td>
          <td>{{ game.opening }}</td>
        </tr>
      </tbody>
      <tfoot>
        <tr>
          <th></th>
          <td>White</td>
          <td>Black</td>
          <td>Result</td>
          <td>Opening</td>
          <th></th>
        </tr>
      </tfoot>
    </table>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useGlobalStore } from "../../stores";

const globalStore = useGlobalStore();

let gameList = computed(() => globalStore.explorer.games);

globalStore.$subscribe((key, value) => {
  gameList = computed(() => globalStore.explorer.games);
});
</script>
