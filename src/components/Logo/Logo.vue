<template>

	<svg
		:width="width"
		:height="height"
		:class="className"
		viewBox="0 0 2160 2160"
		version="1.1"
		xmlns="http://www.w3.org/2000/svg"
		xmlns:xlink="http://www.w3.org/1999/xlink"
		xml:space="preserve"
		xmlns:serif="http://www.serif.com/"
		style="
			fill-rule: evenodd;
			clip-rule: evenodd;
			stroke-linejoin: round;
			stroke-miterlimit: 2;
		"
	>

		<g id="ok-logo">

			<g id="ok-logo-path">

				<path
					fill="currentColor"
					d="M942.403,1397.27c-11.583,-4.533 -35.329,-15.726 -89.744,-54.01c-42.57,-29.95 -334.415,-235.28 -256.117,-598.092c50.609,-234.504 238.794,-327.595 345.861,-364.615l0,211.519c-77.058,19.06 -134.259,88.718 -134.259,171.631c-0,82.913 57.201,152.571 134.259,171.631l0,461.936Z"
				/>

				<path
					fill="currentColor"
					d="M1080.94,912.141l-0.11,0.356c248.01,192.085 262.254,208.289 311.526,271.674c93.131,119.808 102.848,260.505 100.982,255.777l-465.879,0l0,-504.612c19.262,-4.765 37.284,-12.691 53.481,-23.195Z"
				/>

				<path
					fill="currentColor"
					d="M1027.46,353.752l157.332,-62.933l-3.336,123.419l395.274,331.897l-3.336,173.453c-24.797,-17.392 -88.618,-63.317 -119.902,-67.107c-2.601,-0.315 -32.598,-3.95 -20.231,25.751c10.773,25.871 76.54,80.399 87.559,86.957l-42.492,27.783l-95.065,-15.01c-1.811,-2.343 -8.028,-126.968 -88.115,-109.74c-41.743,8.98 -106.675,44.29 -171.661,5.243c23.933,-30.161 38.233,-68.304 38.233,-109.76c-0,-82.913 -57.201,-152.571 -134.26,-171.631l0,-238.322Zm443.804,378.427c17.4,0 31.526,14.126 31.526,31.526c-0,17.399 -14.126,31.525 -31.526,31.525c-17.399,0 -31.525,-14.126 -31.525,-31.525c0,-17.4 14.126,-31.526 31.525,-31.526Zm-244.807,-216.98c26.234,0 47.533,21.299 47.533,47.533c-0,26.234 -21.299,47.533 -47.533,47.533c-26.235,0 -47.533,-21.299 -47.533,-47.533c-0,-26.234 21.298,-47.533 47.533,-47.533Z"
				/>

				<path
					fill="currentColor"
					d="M675.525,1248.35c9.418,12.183 45.607,58.077 58.108,72.144c50.833,57.198 136.17,108.152 150.397,119.451l-233.495,0l24.99,-191.595Z"
				/>

				<path
					fill="currentColor"
					d="M1091.36,1646.72l0.001,0.038l-330.557,0c-0,0 0.206,-46.065 -3.36,-46.699c-78.609,-13.966 -104.148,-91.73 -104.148,-91.73l419.893,-0.816l-0,-0.039l10.104,0.019l10.105,-0.019l-0.001,0.039l400.156,0.778c-0,-0 -25.54,77.764 -104.149,91.73c-3.566,0.633 -3.36,46.699 -3.36,46.699l-294.684,-0Z"
				/>

				<path
					fill="currentColor"
					d="M1199.64,1868.55c-0.001,0.009 -0.002,0.018 -0.002,0.027l-597.471,0c-1.572,7.888 2.772,-63.677 19.766,-99.175c21.685,-45.295 70.766,-56.22 78.376,-57.914c13.322,-2.965 500.172,-4.216 500.172,-4.212c130.139,0.595 236.523,2.063 246.06,4.185c7.61,1.694 56.691,12.619 78.376,57.914c16.995,35.498 21.339,107.063 19.767,99.175l-345.044,0Z"
				/>

			</g>

		</g>

	</svg>

</template>

<script setup lang="ts">
interface Props {
	width?: string | number;
	height?: string | number;
	className?: string;
}

withDefaults(defineProps<Props>(), {
	width: "100%",
	height: "100%",
	className: "",
});
</script>

