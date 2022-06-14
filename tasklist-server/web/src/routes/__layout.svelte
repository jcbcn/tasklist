<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import '../app.css';

	import { page } from '$app/stores';

import { onMount } from 'svelte';

let socket;
let status = 'disconnected';
onMount(() => {
	socket = new WebSocket('ws://localhost:8080/ws');
	socket.addEventListener('open', () => {
		status = 'connected';
		socket.send('test'); //call to get hostname

		//socket.send('tasks');
	});
	socket.addEventListener('close', () => {
		status = 'disconnected';
	});
	socket.addEventListener('error', () => {
		status = 'disconnected';
	});
	socket.addEventListener('message', function (event) {
		//recieve hostname
		status = event.data;
	});
});
</script>

<Header />

<div class="grow min-h-0">
	<!-- Replace with your content -->
	<slot />
	<!-- /End replace -->
</div>

<div class="flex bg-gray-700 py-6 px-4 justify-items-stretch place-items-center">
	<footer class="grow text-gray-400 text-center">
		<p>
			visit <a class="underline" href="https://book.mentality.tools">mentality.tools</a> to learn Tasklist
		</p>
	</footer>
	<div class="justify-self-end">
		<div class="ml-4 flex items-center md:ml-6">
			<a
				href="#"
				class="text-gray-300 bg-gray-700 hover:text-white px-3 py-2 rounded-md text-xs font-small"
				><span class="status uppercase">{status}</span><span
					class={status === 'disconnected' ? 'offline-dot' : 'online-dot'}
				/></a
			>
		</div>
	</div>
</div>

<style>
	.status {
		margin-right: 8px;
	}

	.online-dot {
		height: 10px;
		width: 10px;
		background-color: green;
		border-radius: 50%;
		display: inline-block;
	}

	.offline-dot {
		height: 10px;
		width: 10px;
		background-color: red;
		border-radius: 50%;
		display: inline-block;
	}
</style>
