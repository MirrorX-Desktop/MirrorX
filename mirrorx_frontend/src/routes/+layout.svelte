<script lang="ts">
	import '../app.css';
	import { loadAllLocales } from '../i18n/i18n-util.sync';

	loadAllLocales();

	if (import.meta.env.PROD) {
		document.addEventListener('contextmenu', (event) => event.preventDefault());
	}
</script>

<slot />
