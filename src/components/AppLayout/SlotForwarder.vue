<template>
    <div>
        <slot v-for="(_, name) in $slots" :name="name" :key="name" />
    </div>
</template>

<script>
export default {
    name: 'SlotForwarder',
};
</script>