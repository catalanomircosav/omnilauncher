<script lang="ts">
  const mockGames = [
    { title: "Terraria", platform: "Steam", cover: "https://via.placeholder.com/150x200?text=Terraria" },
    { title: "Bloodborne", platform: "PS4", cover: "https://via.placeholder.com/150x200?text=Bloodborne" },
    { title: "Balatro", platform: "Steam", cover: "https://via.placeholder.com/150x200?text=Balatro" },
    { title: "Clank™", platform: "PS4", cover: "https://via.placeholder.com/150x200?text=Clank" },
  ];

  let filter = $state("All");

  let filteredGames = $derived(
    filter === "All" ? mockGames : mockGames.filter(g => g.platform === filter)
  );
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
    <header class="mb-10 flex justify-between items-end">
      <div>
        <h2 class="text-3xl font-bold text-white mb-1">{filter} Games</h2>
        <p class="text-zinc-400 text-sm">Showing {filteredGames.length} titles</p>
      </div>
      
      <div class="flex gap-4">
        <input 
          type="text" 
          placeholder="Search library..." 
          class="bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-600 transition-all w-64"
        />
      </div>
    </header>

    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-6">
      {#each filteredGames as game}
        <div class="group relative bg-zinc-900 rounded-xl overflow-hidden border border-zinc-800 hover:border-zinc-600 transition-all duration-300 shadow-lg hover:shadow-blue-900/10 hover:-translate-y-1">
          <div class="aspect-[3/4] bg-zinc-800 overflow-hidden relative">
            <img 
              src={game.cover} 
              alt={game.title} 
              class="w-full h-full object-cover opacity-80 group-hover:opacity-100 group-hover:scale-105 transition-all duration-500"
            />
            <div class="absolute inset-0 bg-gradient-to-t from-zinc-950 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex items-end p-4">
              <button class="w-full bg-white text-black font-bold py-2 rounded-lg text-sm shadow-xl active:scale-95 transition-transform">
                Launch
              </button>
            </div>
          </div>
          
          <div class="p-3">
            <h3 class="font-semibold text-sm truncate text-zinc-100">{game.title}</h3>
            <p class="text-xs text-zinc-500 mt-1">{game.platform}</p>
          </div>
        </div>
      {/each}
    </div>
  </main>
</div>

<style>
  :global(body) {
    background-color: #09090b;
    margin: 0;
  }
</style>