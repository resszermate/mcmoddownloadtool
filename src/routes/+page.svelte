<script lang="ts">
let files: FileList | null = null;
</script>



<main class="container">
  <h1>Minecraft mod downloader tool</h1>

  <div class="row"></div>
      <img src="/mc512.png" class="logo mc" alt="Minecraft Logo" />

  <h2>Select your mod folder</h2>
  <input bind:files id="dir" multiple type="file" webkitdirectory>

  {#if files}
  <table>
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
  {/if}
  


</main>

<style>


</style>
