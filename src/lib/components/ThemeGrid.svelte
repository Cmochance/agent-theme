<script lang="ts">
  import { themesStore } from '../stores';
  import ThemeCard from './ThemeCard.svelte';
  import UploadCard from './UploadCard.svelte';
</script>

<div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-[24px]">
  {#each $themesStore as theme (theme.id)}
    <ThemeCard {theme} />
  {/each}
  <UploadCard />
</div>
