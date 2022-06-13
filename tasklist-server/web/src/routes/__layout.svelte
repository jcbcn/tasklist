<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import '../app.css';
</script>


<div class="min-h-full">

	<Header />
  
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
		<p>visit <a class="underline" href="https://book.mentality.tools">mentality.tools</a> to learn Tasklist</p>
	</footer>
  </div>

