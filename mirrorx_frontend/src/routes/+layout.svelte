<script lang="ts">
	import '../app.css';
	import { loadAllLocales } from '../i18n/i18n-util.sync';

	loadAllLocales();
</script>

<slot />
