<script lang="ts">
  let number_of_controllers = $state(1);
  let controllerIds: string[] = $state([]);
  let current_controller_id = $state("");

  $effect(() => {
    number_of_controllers = Math.min(Math.max(number_of_controllers, 1), 16)
  })

  let button_class = "outline-1 p-2 cursor-pointer hover:bg-black hover:text-white transition-all"

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

  async function pressButton(event: Event) {
    if (!controllerIds.includes(current_controller_id)) {
        return
    }

    let action_id
    if (event.target instanceof Element) {
        action_id = event.target.id
    } else {
        return
    }

    fetch(
      "http://localhost:3000/controllers/input",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          controller_id: current_controller_id,
          action_id: action_id,
        }),
      }
    )
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
  <button class={`rounded ${button_class}`} onclick={createControllers}>Create Controllers</button>
  <p>{controllerIds.join(" | ")}</p>

  <input type="text" class="outline-1 rounded p-2 w-32 text-center" bind:value={current_controller_id}/>
  <div class="flex justify-around gap-2">
    <button id="a" onclick={pressButton} class={`w-10 rounded-full ${button_class}`}>A</button>
    <button id="b" onclick={pressButton} class={`w-10 rounded-full ${button_class}`}>B</button>
    <button id="x" onclick={pressButton} class={`w-10 rounded-full ${button_class}`}>X</button>
    <button id="y" onclick={pressButton} class={`w-10 rounded-full ${button_class}`}>Y</button>
  </div>
  <div class="flex justify-around gap-2">
    <button id="dpad_left" onclick={pressButton} class={`rounded-full ${button_class} w-16`}>Left</button>
    <button id="dpad_up" onclick={pressButton} class={`rounded-full ${button_class} w-16`}>Up</button>
    <button id="dpad_down" onclick={pressButton} class={`rounded-full ${button_class} w-16`}>Down</button>
    <button id="dpad_right" onclick={pressButton} class={`rounded-full ${button_class} w-16`}>Right</button>
  </div>
</main>
