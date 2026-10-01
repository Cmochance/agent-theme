<script lang="ts">
  export let checked: boolean = false;
  export let disabled: boolean = false;
  export let onChange: (value: boolean) => void = () => {};
</script>

<label
  class="relative inline-block h-[22px] w-[38px] shrink-0"
  class:opacity-50={disabled}
>
  <input
    type="checkbox"
    bind:checked
    {disabled}
    on:change={() => onChange(checked)}
    class="sr-only"
  />
  <span
    class="absolute inset-0 rounded-full transition-colors duration-200"
    style="background:{checked ? 'var(--green)' : 'var(--fill-active)'};"
  ></span>
  <span
    class="absolute left-[2px] top-[2px] h-[18px] w-[18px] rounded-full bg-white transition-transform duration-200"
    style="box-shadow:0 1px 2px rgba(0,0,0,0.25);{checked ? ' transform:translateX(16px);' : ''}"
  ></span>
</label>
