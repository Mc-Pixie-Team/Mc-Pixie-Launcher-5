<script lang="ts">


import Card from "~/components/card.vue";

export default {
  components: {Card},

  async setup() {
    let offset = 0;
    const refdata = ref<any[]>([]);

    async function makeModrinthReqeust(offset: number): Promise<any[]> {
      try {
        const facets = encodeURIComponent('[["project_type:modpack"]]');
        const url = `https://api.modrinth.com/v2/search?offset=${offset}&index=relevance&limit=50&facets=${facets}`;

        const {data, status, error, refresh, clear} = await useFetch(url);
        if (error.value) throw error.value;
        return (data.value as any)["hits"];
      } catch (e: any) {
        alert(e.message);
        return [];
      }
    }


    refdata.value = await makeModrinthReqeust(offset);


    const {list, containerProps, wrapperProps} = useVirtualList(refdata, {
      itemHeight:150
    });

    useInfiniteScroll(containerProps.ref, async () => {
      offset += 50;
      let data = await makeModrinthReqeust(offset);
      refdata.value.push(...data);

    }, {distance: 200,})

    return {
      containerProps,
      list,
      wrapperProps
    }
  }
}
</script>

<template>
  <div v-bind="containerProps">
    <div v-bind="wrapperProps">
      <Card v-for="{ index, data } in list"
            :key="index" :title="data['title']" :icon_url="data['icon_url']"  />
    </div>
  </div>
</template>

<style scoped>

</style>