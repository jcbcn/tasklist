<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import '../app.css';
</script>


<div class="min-h-full">

	<Header />
  
	<header class="bg-white shadow">
	  <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
		<h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
	  </div>
	</header>
	<main>
	  <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
		<!-- Replace with your content -->
		<main>
			<slot />
		</main>
		<!-- /End replace -->
	  </div>
	</main>
	<footer class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8 text-gray-400">
		<p>visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
  </div>

