<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let number_of_controllers = $state(1);
  let controllerIds: string[] = $state([]);

  $effect(() => {
    number_of_controllers = Math.min(Math.max(number_of_controllers, 1), 16)
  })

  async function createControllers(event: Event) {
    event.preventDefault();
    controllerIds = await fetch(
      "http://localhost:3000/controllers",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          number_of_controllers: number_of_controllers
        }),
      }
    ).then(response => response.json())
  }
</script>

<main class="flex flex-col w-screen h-screen justify-center items-center gap-5">
  <div class="flex gap-5">
    <input
      id="greet-input"
      type="range"
      min="1"
      max="16"
      placeholder="Enter a name..."
      bind:value={number_of_controllers}
    />
    <p class="w-10 text-center">{number_of_controllers}</p>
  </div>
  <button class="outline-1 rounded p-2 cursor-pointer hover:bg-black hover:text-white transition-all" onclick={createControllers}>Create Controllers</button>
  <p>{controllerIds.join(" | ")}</p>
</main>
