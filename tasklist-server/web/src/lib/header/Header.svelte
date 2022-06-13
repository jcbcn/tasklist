<script lang="ts">
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

<header>
	<nav class="bg-gray-800">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="flex items-center justify-between h-16">
				<div class="flex items-center">
					<div class="flex-shrink-0">
						<img
							class="h-8 w-8"
							src="https://tailwindui.com/img/logos/workflow-mark-indigo-500.svg"
							alt="Workflow"
						/>
					</div>
					<div class="hidden md:block">
						<div class="ml-10 flex items-baseline space-x-4">
							<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
							<a
								href="#"
								class="bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium"
								aria-current="page">Dashboard</a
							>

							<a
								href="#"
								class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
								>Team</a
							>

							<a
								href="#"
								class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
								>Projects</a
							>

							<a
								href="#"
								class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
								>Calendar</a
							>

							<a
								href="#"
								class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
								>Reports</a
							>
						</div>
					</div>
				</div>
				<div class="hidden md:block">
					<div class="ml-4 flex items-center md:ml-6">
						<a href="#" class="text-gray-300 bg-gray-700 hover:text-white px-3 py-2 rounded-md text-xs font-small"
							><span class="status uppercase">{status}</span><span
								class={status === 'disconnected' ? 'offline-dot' : 'online-dot'}
							/></a
						>
					</div>
				</div>
				<div class="-mr-2 flex md:hidden">
					<!-- Mobile menu button -->
					<button
						type="button"
						class="bg-gray-800 inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white"
						aria-controls="mobile-menu"
						aria-expanded="false"
					>
						<span class="sr-only">Open main menu</span>
						<!--
				  Heroicon name: outline/menu
	
				  Menu open: "hidden", Menu closed: "block"
				-->
						<svg
							class="block h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 6h16M4 12h16M4 18h16"
							/>
						</svg>
						<!--
				  Heroicon name: outline/x
	
				  Menu open: "block", Menu closed: "hidden"
				-->
						<svg
							class="hidden h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					</button>
				</div>
			</div>
		</div>
	</nav>
</header>

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
