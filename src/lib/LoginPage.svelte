<script lang="ts">
    import Checkbox from "./Checkbox.svelte";
    import { login, saveCredentials } from "./scc";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    let errorMessage: string = "";

    async function handleLogin(event: SubmitEvent) {
      errorMessage = "";
      
      const data = new FormData(event.target as HTMLFormElement);
      const entries = [...data.entries()];
      const remember = (document.querySelector("button[name=remember]") as HTMLButtonElement)?.value == "true";
      const username: string = entries[0][1] as string;
      const password: string = entries[1][1] as string;
    
      const [success, msg] = await login(username, password);
      if (!success) {
        console.error(`Error logging in: ${msg}`);
        errorMessage = matchMessage(msg);
        return;
      }
      errorMessage = "";

      console.log(`Logged in as ${username}`);
      if (remember) {
        if (await saveCredentials(username, password)) {
            console.log("Saved credentials!");
          } else {
            console.error("Failed to save credentials!");
          }
      }
      dispatch("login", { username: username, token: msg });
  }

  function matchMessage(msg: string): string {
    switch (msg) {
      case "LOGIN_FATAL_1":
        return "Invalid username or password!";
      default:
        return "An unknown error occurred!";
    }
  }
</script>

<div class="flex flex-col content-start p-0 min-h-screen">
    <div class="glassmorphic flex items-center justify-center rounded-md grow m-5">
      <form on:submit|preventDefault={handleLogin} class="flex flex-col gap-2 items-start">
        <input name="username" type="text" class="w-64 glassmorphic bg-transparent place-self-center rounded-md px-3 py-2 text-white placeholder-gray-200 focus:ring-2" placeholder="Username"/>
        <input name="password" type="password" class="w-64 glassmorphic bg-transparent place-self-center rounded-md px-3 py-2 text-white placeholder-gray-200 focus:ring-2" placeholder="Password"/>
        <Checkbox text={"Stay logged in"} />
        <button type="submit" class="glassmorphic bg-transparent place-self-end rounded-md px-2 py-1 text-white border-2 border-white">Login</button>
        <p class="text-red-500 text-base xl:text-lg">{errorMessage}</p>
      </form>
    </div>
</div>