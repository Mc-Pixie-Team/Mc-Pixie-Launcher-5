<script setup lang="ts">

</script>

<template>

  <div data-tauri-drag-region class=" h-8 content-center justify-end flex flex-row">

    <div class="h-full flex justify-center w-[46px] hover:bg-black " >
    <img
        src="~/assets/svg/minimize.svg"
        alt="minimize"
        class="w-[10px] "
    />
    </div>
    <div class="h-full flex justify-center w-[46px] hover:bg-black">
    <img
        class="w-[11px] "
        src="~/assets/svg/maximizebutton.svg"
        alt="maximize"
    />
    </div>
      <div class="h-full flex justify-center w-[46px] hover:bg-black">
    <img
        src="~/assets/svg/closebutton.svg"
        alt="close"
        class="w-[11px]  "
    />
      </div>


  </div>
</template>

<style scoped>


.titlebar {
  height: 30px;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  background-color: aquamarine;

}

.titlebar-buttons {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}

.titlebar-button:hover {
  background: #5bbec3;
}
</style>