<script lang="ts">
  export let checked: boolean = false;
  export let disabled: boolean = false;
  export let onChange: (value: boolean) => void = () => {};
</script>

<label class="relative inline-block w-[48px] h-[24px]">
  <input
    type="checkbox"
    bind:checked
    {disabled}
    on:change={() => onChange(checked)}
    class="opacity-0 w-0 h-0"
  />
  <span
    class="slider absolute cursor-pointer inset-0 rounded-[24px] transition-all duration-300
      shadow-[inset_0_2px_4px_rgba(0,0,0,0.15),0_1px_0_rgba(255,255,255,0.5)]
      {checked
        ? 'bg-[linear-gradient(180deg,#6ee7b7_0%,#22c55e_100%)] border-[rgba(34,197,94,0.4)]'
        : 'bg-[linear-gradient(180deg,#d1d5db_0%,#9ca3af_100%)] border-[rgba(100,110,130,0.3)]'}"
  >
    <span
      class="absolute content-[''] h-[16px] w-[16px] left-[3px] bottom-[3px]
        bg-[radial-gradient(circle_at_40%_35%,#ffffff,#e5e7eb)]
        shadow-[0_1px_3px_rgba(0,0,0,0.25),inset_0_1px_1px_rgba(255,255,255,0.8)]
        rounded-full transition-all duration-300"
      style={checked ? 'transform: translateX(24px);' : ''}
    ></span>
  </span>
</label>
