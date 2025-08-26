<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let files: FileList | null = null;

    async function getModNames(){
    await invoke("get_mod_names", { fileList: { files_list: Array.from(files!).map(f => f.name) } });
  }
  

</script>



<main class="container">
  <h1>Minecraft mod downloader tool</h1>

  <div class="row"></div>
      <img src="/mc512.png" class="logo mc" alt="Minecraft Logo" />

  <h2>Select your mod folder</h2>
  <input bind:files id="dir" multiple type="file" webkitdirectory>

  {#if files}
  <table class="mods-table">
    <thead>
      <tr>
        <td>Name</td>
        <td>Size</td>
      </tr>
    </thead>
    <tbody>
  {#each Array.from(files ? Array.from(files) : []) as file}
    <tr>
    <td>{file.name}</td>
    <td>{file.size}</td>
    </tr>
  {/each}
  </tbody>
  </table>

  <button on:click={getModNames}>Download files from Modrinth</button>
  {/if}
  


</main>

<style>


</style>
