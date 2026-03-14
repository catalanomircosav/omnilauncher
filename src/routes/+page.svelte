<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Game {
    id?: number;
    game_id: string;
    description?: string;
    title: string;
    platform: string;
    executable_path: string;
    cover_url?: string;
  }

  // Svelte 5 Runes
  let games = $state<Game[]>([]);
  let filter = $state("All");
  let isLoading = $state(true);

  // Filtro derivato dai dati reali del backend
  let filteredGames = $derived(
    filter === "All" ? games : games.filter(g => g.platform === filter)
  );

  async function loadGames() {
    try {
      isLoading = true;
      // Chiamata al comando Rust get_games (Issue #3)
      games = await invoke<Game[]>("get_games");
    } catch (error) {
      console.error("Errore nel caricamento dei giochi:", error);
    } finally {
      isLoading = false;
    }
  }

  async function launchGame(game: Game) {
    try {
      if (game.platform === "Steam") {
        await invoke("launch_steam_game", { gameId: game.game_id });
      } else if (game.platform === "PS4") {
        await invoke("launch_shadps4_game", { gameId: game.game_id });
      }
    } catch (error) {
      console.error("Errore durante l'avvio:", error);
    }
  }

  onMount(loadGames);
</script>

<div class="flex h-screen bg-zinc-950 text-zinc-100 font-sans">
  <aside class="w-64 border-r border-zinc-800 p-6 flex flex-col gap-8 bg-zinc-900/50">
    <div class="flex items-center gap-3 px-2">
      <div class="w-8 h-8 bg-blue-600 rounded-lg shadow-[0_0_15px_rgba(37,99,235,0.4)]"></div>
      <h1 class="text-xl font-bold tracking-tight text-white">OmniLauncher</h1>
    </div>

    <nav class="flex flex-col gap-2">
      <p class="text-[10px] uppercase tracking-widest text-zinc-500 font-bold px-2 mb-2">Library</p>
      {#each ["All", "Steam", "PS4"] as platform}
        <button
          onclick={() => filter = platform}
          class="flex items-center px-3 py-2 rounded-md transition-all duration-200 group
          {filter === platform ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
        >
          <span class="text-sm font-medium">{platform}</span>
          {#if filter === platform}
            <div class="ml-auto w-1.5 h-1.5 bg-blue-500 rounded-full"></div>
          {/if}
        </button>
      {/each}
    </nav>
  </aside>

  <main class="flex-1 overflow-y-auto p-8">
    {#if isLoading}
      <div class="h-full flex items-center justify-center">
        <p class="text-zinc-500 animate-pulse">Scanning library...</p>
      </div>
    {:else}
      <header class="mb-10 flex justify-between items-end">
        <div>
          <h2 class="text-3xl font-bold text-white mb-1">{filter} Games</h2>
          <p class="text-zinc-400 text-sm">Found {filteredGames.length} titles in your database</p>
        </div>
      </header>

      <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-6">
        {#each filteredGames as game}
          <div class="group relative bg-zinc-900 rounded-xl overflow-hidden border border-zinc-800 hover:border-zinc-500 transition-all duration-300">
            <div class="aspect-[3/4] bg-zinc-800 relative">
              <img 
                src={game.cover_url || `https://via.placeholder.com/300x400/18181b/ffffff?text=${game.title}`} 
                alt={game.title} 
                class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
              />
              <div class="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center p-6">
                <button 
                  onclick={() => launchGame(game)}
                  class="bg-blue-600 hover:bg-blue-500 text-white font-bold py-3 px-6 rounded-full shadow-2xl transform translate-y-4 group-hover:translate-y-0 transition-all duration-300"
                >
                  PLAY
                </button>
              </div>
            </div>
            
            <div class="p-3 bg-zinc-900">
              <h3 class="font-semibold text-sm truncate text-zinc-100">{game.title}</h3>
              <div class="flex items-center gap-2 mt-1">
                <span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400 border border-zinc-700">
                  {game.platform}
                </span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </main>
</div>